use holochain_manager::config::LaunchHolochainConfig;
use holochain_manager::errors::{LaunchHolochainError, InitializeConductorError};
use holochain_web_app_manager::error::LaunchWebAppManagerError;
use lair_keystore_manager::error::{LairKeystoreError, LaunchChildError};
use lair_keystore_manager::utils::create_dir_if_necessary;
use lair_keystore_manager::versions::v0_2::LairKeystoreManagerV0_2;
use lair_keystore_manager::LairKeystoreManager;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use sysinfo::{System, SystemExt};
use tauri::api::process::Command;
use tauri::{AppHandle, Manager};
use tauri::{window::WindowBuilder, WindowUrl};
use tauri::{CustomMenuItem, Menu, Submenu};

use std::fs::{read};

use holochain_manager::versions::HolochainVersion;
use holochain_web_app_manager::WebAppManager;

use crate::file_system::{
  config_environment_path, data_path_for_holochain_version, keystore_data_path, root_config_path,
  root_holochain_data_path, root_lair_path,
};
use crate::system_tray::AllInstalledApps;
use crate::{running_state::RunningState, system_tray::update_system_tray, LauncherState};

use super::config::LauncherConfig;
use super::default_apps::install_default_apps_if_necessary;
use super::error::LauncherError;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content")]
pub enum KeystoreStatus {
  InitNecessary,
  PasswordNecessary,
  LaunchKeystoreError(LairKeystoreError),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content")]
pub enum HolochainId {
  HolochainVersion(HolochainVersion),
  CustomBinary,
}

pub struct LauncherManager {
  app_handle: AppHandle,
  config: LauncherConfig,

  pub holochain_managers:
    HashMap<HolochainVersion, RunningState<WebAppManager, LaunchWebAppManagerError>>,
  pub custom_binary_manager: Option<RunningState<WebAppManager, LaunchWebAppManagerError>>,
  pub lair_keystore_manager: RunningState<Box<dyn LairKeystoreManager>, KeystoreStatus>,
}

impl LauncherManager {
  pub async fn launch(app_handle: AppHandle) -> Result<Self, LauncherError> {
    create_dir_if_necessary(&root_lair_path())?;
    create_dir_if_necessary(&root_holochain_data_path())?;
    create_dir_if_necessary(&root_config_path())?;

    let keystore_path = keystore_data_path(LairKeystoreManagerV0_2::lair_keystore_version());

    let is_initialized = LairKeystoreManagerV0_2::is_initialized(keystore_path);

    let keystore_status = match is_initialized {
      true => KeystoreStatus::PasswordNecessary,
      false => KeystoreStatus::InitNecessary,
    };

    let config = LauncherConfig::read();

    let app_handle2 = app_handle.clone();
    let manager = LauncherManager {
      app_handle: app_handle.clone(),
      holochain_managers: HashMap::new(),
      custom_binary_manager: None,
      config,
      lair_keystore_manager: RunningState::Error(keystore_status),
    };

    // This doesn't work... TODO: Fix it
    app_handle.listen_global("running_apps_changed", move |_| {
      let launcher_state: &LauncherState = &app_handle2.state();
      let result = tauri::async_runtime::block_on(async move {
        let mut mutex = (*launcher_state).lock().await;
        let manager = mutex.get_running()?;
        manager.on_apps_changed().await
      });

      if let Err(err) = result {
        log::error!("Couldn't refresh apps: {:?}", err);
      }
    });

    Ok(manager)
  }

  pub async fn initialize_and_launch_keystore(&mut self, password: String) -> Result<(), String> {
    let keystore_path = keystore_data_path(LairKeystoreManagerV0_2::lair_keystore_version());

    LairKeystoreManagerV0_2::initialize(keystore_path, password.clone())
      .await
      .map_err(|err| format!("Error initializing the keystore: {:?}", err))?;

    self.launch_keystore(password).await?;

    Ok(())
  }

  pub async fn launch_keystore(&mut self, password: String) -> Result<(), String> {
    let keystore_path = keystore_data_path(LairKeystoreManagerV0_2::lair_keystore_version());
    let lair_keystore_manager =
      LairKeystoreManagerV0_2::launch(self.config.log_level, keystore_path, password.clone())
        .await
        .map_err(|err| format!("Error launching the keystore: {:?}", err))?;

    self.lair_keystore_manager = RunningState::Running(Box::new(lair_keystore_manager));

    let mut holochain_versions_to_run = self.config.running_versions.clone();

    holochain_versions_to_run.insert(HolochainVersion::default());

    for version in holochain_versions_to_run {
      self.launch_holochain_manager(version, None).await?;
    }

    if let Some(path) = self.config.custom_binary_path.clone() {
      self
        .launch_holochain_manager(HolochainVersion::custom(), Some(path))
        .await?;
    } else {
      let _r = std::fs::remove_dir_all(root_config_path().join("custom"));
      let _r = std::fs::remove_dir_all(root_holochain_data_path().join("custom"));
    }

    Ok(())
  }

  pub async fn launch_holochain_manager(
    &mut self,
    version: HolochainVersion,
    custom_binary_path: Option<String>,
  ) -> Result<(), String> {
    // If we are trying to launch Holochain from a custom binary path, but there is nothing in that path, error and exit immediately
    if let Some(path) = custom_binary_path.clone() {
      if !Path::new(&path).exists() {
        self.custom_binary_manager = Some(RunningState::Error(
          LaunchWebAppManagerError::LaunchHolochainError(LaunchHolochainError::LaunchChildError(
            LaunchChildError::BinaryNotFound,
          )),
        ));
        return Ok(());
      }
    }

    let admin_port = portpicker::pick_unused_port().expect("No ports free");

    let conductor_config_path = match custom_binary_path.is_some() {
      true => root_config_path().join("custom"),
      false => config_environment_path(version),
    };
    let environment_path = match custom_binary_path.is_some() {
      true => root_holochain_data_path().join("custom"),
      false => data_path_for_holochain_version(version),
    };

    let lair_manager = self.get_lair_keystore_manager()?;

    let keystore_connection_url = lair_manager.connection_url();
    let password = lair_manager.password();

    let command = match custom_binary_path.clone() {
      Some(p) => Ok(Command::new(p)),
      None => {
        let version_str: String = version.into();
        Command::new_sidecar(format!("holochain-v{}", version_str))
          .map_err(|err| format!("{}", err))
      }
    }?;

    let config = LaunchHolochainConfig {
      log_level: self.config.log_level,
      command,
      admin_port,
      config_environment_path: conductor_config_path,
      environment_path,
      keystore_connection_url,
    };

    let version_str: String = version.into();

    let state = match WebAppManager::launch(version, config, password).await {
      Ok(mut manager) => match version.eq(&HolochainVersion::default()) {
        true => match install_default_apps_if_necessary(&mut manager).await {
          Ok(()) => {
            log::info!("Launched Holochain {}", version_str);
            RunningState::Running(manager)
          }
          Err(err) => {
            manager.kill()?;
            log::error!(
              "Error launching Holochain {}: Could not install default apps: {}",
              version_str,
              err
            );

            RunningState::Error(LaunchWebAppManagerError::Other(format!(
              "Could not install default apps: {}",
              err
            )))
          }
        },
        false => {
          let version_str: String = version.into();
          log::info!("Launched Holochain {}", version_str);
          RunningState::Running(manager)
        }
      },
      Err(error) => {
        log::error!("Error launching Holochain {}: {}", version_str, error);
        match error.clone() {
          LaunchWebAppManagerError::LaunchHolochainError(LaunchHolochainError::CouldNotInitializeConductor(ie)) => {
            match ie {
              InitializeConductorError::SqliteError(e) => {
                if e.contains("DatabaseError(SqliteError(SqliteFailure(Error { code: NotADatabase, extended_code: 26 }") {
                  self.app_handle.emit_all("WrongDatabaseFileType", ())
                    .map_err(|e| format!("Failed to send WrongDatabaseFileType error to frontend: {}", e))?;
                }
              },
              _ => (),
            }
          },
          _ => (),
        };
        RunningState::Error(error)
      }
    };


    if custom_binary_path.is_some() {
      self.custom_binary_manager = Some(state);
    } else {
      self.holochain_managers.insert(version.clone(), state);
      self.config.running_versions.insert(version);
    }

    self
      .config
      .write()
      .map_err(|err| format!("Could not write launcher config: {}", err))?;

    self.on_apps_changed().await?;

    Ok(())
  }

  pub fn get_lair_keystore_manager(&mut self) -> Result<&Box<dyn LairKeystoreManager>, String> {
    match &self.lair_keystore_manager {
      RunningState::Running(m) => Ok(m),
      RunningState::Error(error) => Err(format!(
        "This lair-keystore version is not running: {:?}",
        error
      )),
    }
  }

  pub async fn get_or_launch_holochain(
    &mut self,
    holochain_id: HolochainId,
  ) -> Result<&mut WebAppManager, String> {
    match holochain_id {
      HolochainId::HolochainVersion(version) => {
        if let None = self.holochain_managers.get(&version) {
          self.launch_holochain_manager(version.clone(), None).await?;
        }
      }
      HolochainId::CustomBinary => {
        let path = self
          .config
          .custom_binary_path
          .clone()
          .ok_or(String::from("There is no custom binary path specified"))?;

        if let None = self.custom_binary_manager {
          self
            .launch_holochain_manager(HolochainVersion::custom(), Some(path))
            .await?;
        }
      }
    }

    self.get_web_happ_manager(holochain_id)
  }

  pub fn get_web_happ_manager(
    &mut self,
    holochain_id: HolochainId,
  ) -> Result<&mut WebAppManager, String> {
    let manager_state = match holochain_id {
      HolochainId::HolochainVersion(version) => self
        .holochain_managers
        .get_mut(&version)
        .ok_or(String::from("This holochain version is not running")),
      HolochainId::CustomBinary => self.custom_binary_manager.as_mut().ok_or(String::from(
        "There is no Holochain running with custom binary",
      )),
    }?;

    match manager_state {
      RunningState::Running(m) => Ok(m),
      RunningState::Error(error) => Err(format!(
        "This holochain version is not running: {:?}",
        error
      )),
    }
  }

  /// Connects to the conductor, requests the list of running apps and updates the system tray
  pub async fn on_apps_changed(&mut self) -> Result<(), String> {
    let versions: Vec<HolochainVersion> = self.holochain_managers.keys().cloned().collect();

    let mut all_installed_apps = AllInstalledApps {
      by_version: HashMap::new(),
      custom_binary: None,
    };

    for version in versions {
      if let Ok(manager) = self.get_web_happ_manager(HolochainId::HolochainVersion(version.clone()))
      {
        let running_apps = manager.list_apps().await?;

        all_installed_apps
          .by_version
          .insert(version.clone(), running_apps);
      }
    }

    if let Some(m) = &mut self.custom_binary_manager {
      match m.get_running() {
        Ok(manager) => {
          let running_apps = manager.list_apps().await?;

          all_installed_apps.custom_binary = Some(running_apps);
        }
        Err(_) => {}
      }
    }

    update_system_tray(&self.app_handle, &all_installed_apps);

    // TODO: Iterate over the open windows, close any that has been uninstalled/disabled

    Ok(())
  }

  pub fn open_app(&mut self, holochain_id: HolochainId, app_id: &String) -> Result<(), String> {
    let mut window_label = app_id.clone().replace("-", "--").replace(" ", "-").replace(".", "_");
    window_label.push_str("---EXTERNAL"); // !! this line is required for security reasons, to unambiguously differentiate the this window from the admin window

    // Iterate over the open windows, focus if the app is already open

    if let Some(w) = self.app_handle.get_window(window_label.as_str()) {
      if let Err(err) = w.show() {
        log::error!("Error showing the window: {:?}", err);
      }

      return Ok(());
    }

    let manager = self.get_web_happ_manager(holochain_id)?;

    let index_path = manager.get_ui_index_path(app_id);
    let assets_path = manager.get_app_ui_path(app_id);


    // println!("%*%*%*% INDEX PATH: {:?}", index_path);


    let launcher_env = format!(r#"{{
        "APP_INTERFACE_PORT": {},
        "ADMIN_INTERFACE_PORT": {},
        "INSTALLED_APP_ID": "{}"
      }}"#,
      manager.holochain_manager.app_interface_port(),
      manager.holochain_manager.admin_interface_port(),
      app_id
    );

    let window = WindowBuilder::new(
      &self.app_handle,
      window_label.clone(),
      WindowUrl::App("index.html".into())
    )
    .on_web_resource_request(move |request, response| {
      // println!("£*£*£*£* REQUEST BEFORE {:?}", request);
      let uri = request.uri();
      match uri {
        "tauri://localhost" => {
          let mutable_response = response.body_mut();
          match read(index_path.clone()) {
            Ok(index_html) => *mutable_response = index_html, // TODO! Check if there are better ways of dealing with errors here
            Err(e) => log::error!("Error reading the path of the UI's index.html: {:?}", e),
          }
        },
        "tauri://localhost/.launcher-env.json" => {
          let mutable_response = response.body_mut();
          *mutable_response = launcher_env.as_bytes().to_vec();
          response.set_mimetype(Some(String::from("application/json")));
        },
        _ => {
          if uri.starts_with("tauri://localhost/") {

            let mut asset_file = &uri[18..]; // TODO! proper error handling. index may be out of bounds?

            // if uri is exactly "tauri://localhost/" redirect to index.html (otherwise it will try to redirect to the admin window's index.html)
            if asset_file == "" {
              asset_file = "index.html";
            }

            let mime_guess = mime_guess::from_path(asset_file);

            let mime_type = match mime_guess.first() {
              Some(mime) => Some(mime.essence_str().to_string()),
              None => {
                log::info!("Could not deterine MIME Type of file '{:?}'", asset_file);
                None
              }
            };

            // println!("%#%#%# ASSEETTT: {:?}", asset_file);
            // println!("%#%#%# Mime type: {:?}", mime_type);
            let asset_path = assets_path.join(asset_file);
            // println!("%#%#%# ASSEETTT PATH: {:?}", asset_path);
            match read(asset_path.clone()) {
              Ok(asset) => {
                let mutable_response = response.body_mut();
                *mutable_response = asset;
                response.set_mimetype(mime_type);
              },
              Err(e) => log::error!("Error reading asset file from path '{:?}'. Error: {:?}", asset_path, e),
            }
          }
        }
      }


    })
    .inner_size(1000.0, 700.0)
    .title(app_id)
    .enable_clipboard_access() // TODO! potentially make this optional
    .menu(Menu::new().add_submenu(Submenu::new(
      "Settings",
      Menu::new().add_item(CustomMenuItem::new("show-devtools", "Show DevTools")),
    )))
    .build()
    .map_err(|err| format!("Error opening app: {:?}", err))?;

    let a = self.app_handle.clone();
    let l = window_label.clone();
    window.on_menu_event(move |_| {
      if let Some(w) = a.get_window(l.as_str()) {
        w.open_devtools();
      }
    });

    Ok(())
  }

  pub fn is_launcher_already_running() -> bool {
    let s = System::new_all();
    for _ in s.processes_by_name("holochain-launcher") {
      return true;
    }
    return false;
  }
}
