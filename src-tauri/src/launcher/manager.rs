use holochain_launcher_utils::window_builder::happ_window_builder;
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
  conductor_config_dir, holochain_version_data_dir, keystore_data_dir,
  profile_holochain_data_dir, profile_lair_dir, profile_config_dir, Profile, launcher_config_dir,
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
  pub profile: String,
}

impl LauncherManager {
  pub async fn launch(app_handle: AppHandle, profile: Profile) -> Result<Self, LauncherError> {

    create_dir_if_necessary(&profile_lair_dir(profile.clone())?)?;
    create_dir_if_necessary(&profile_holochain_data_dir(profile.clone())?)?;
    create_dir_if_necessary(&profile_config_dir(profile.clone())?)?;
    create_dir_if_necessary(&launcher_config_dir(profile.clone())?)?;

    let keystore_path = keystore_data_dir(LairKeystoreManagerV0_2::lair_keystore_version(), profile.clone())?;

    let is_initialized = LairKeystoreManagerV0_2::is_initialized(keystore_path);

    let keystore_status = match is_initialized {
      true => KeystoreStatus::PasswordNecessary,
      false => KeystoreStatus::InitNecessary,
    };

    let config = LauncherConfig::read(profile.clone());

    let app_handle2 = app_handle.clone();
    let manager = LauncherManager {
      app_handle: app_handle.clone(),
      holochain_managers: HashMap::new(),
      custom_binary_manager: None,
      config,
      lair_keystore_manager: RunningState::Error(keystore_status),
      profile,
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

  /// Initializes a new keystore with the given password, then lanuches LairKeystoreManager, HolochainManager(s)
  /// and WebAppManager(s).
  pub async fn initialize_keystore_and_launch(&mut self, password: String, profile: Profile) -> Result<(), String> {

    // emitting signal to the front-end for progress indication
    self.app_handle.get_window("admin").unwrap()
      .emit("progress-update", String::from("Initializing keystore"))
      .map_err(|e| format!("Failed to send signal to the frontend: {:?}", e))?;

    let keystore_path = keystore_data_dir(LairKeystoreManagerV0_2::lair_keystore_version(), profile.clone())
      .map_err(|e| format!("Failed to get keystore data dir: {}", e))?;

    LairKeystoreManagerV0_2::initialize(keystore_path, password.clone())
      .await
      .map_err(|err| format!("Error initializing the keystore: {:?}", err))?;

    // emitting signal to the front-end for progress indication
    self.app_handle.get_window("admin").unwrap()
      .emit("progress-update", String::from("Launching keystore"))
      .map_err(|e| format!("Failed to send signal to the frontend: {:?}", e))?;

    self.launch_managers(password, profile).await?;

    Ok(())
  }


  /// Launches LairKeystoreManager, HolochainManager(s) and WebAppManager(s).
  pub async fn launch_managers(&mut self, password: String, profile: Profile) -> Result<(), String> {

    let keystore_path = keystore_data_dir(LairKeystoreManagerV0_2::lair_keystore_version(), profile.clone())
      .map_err(|e| format!("Failed to get keystore data dir: {}", e))?;

    let lair_keystore_manager =
      LairKeystoreManagerV0_2::launch(self.config.log_level, keystore_path, password.clone())
        .await
        .map_err(|err| format!("Error launching the keystore: {:?}", err))?;

    self.lair_keystore_manager = RunningState::Running(Box::new(lair_keystore_manager));

    let mut holochain_versions_to_run = self.config.running_versions.clone();

    holochain_versions_to_run.insert(HolochainVersion::default());

    for version in holochain_versions_to_run {
      // emitting signal to the front-end for progress indication
      self.app_handle.get_window("admin").unwrap()
        .emit("progress-update", format!("Launching Holochain version {}", version.to_string()))
        .map_err(|e| format!("Failed to send signal to the frontend: {:?}", e))?;

      self.launch_holochain_manager(version, None, profile.clone()).await?;
    }

    if let Some(path) = self.config.custom_binary_path.clone() {
      self
        .launch_holochain_manager(HolochainVersion::custom(), Some(path), profile.clone())
        .await?;
    } else {
      // If no custom holochain binary is specified in launcher-config.yaml, remove the data associated to previous
      // custom holochain binaries
      let _r = std::fs::remove_dir_all(profile_config_dir(profile.clone())
        .map_err(|e| format!("Failed to get profile config dir: {}", e))?
        .join("custom"));
      let _r = std::fs::remove_dir_all(profile_holochain_data_dir(profile)
        .map_err(|e| format!("Failed to get profile holochain data dir: {}", e))?
        .join("custom"));
    }

    Ok(())
  }


  /// Launches a Holochain Manager and the associated WebAppManager for a given holochain version
  /// or custom binary
  pub async fn launch_holochain_manager(
    &mut self,
    version: HolochainVersion,
    custom_binary_path: Option<String>,
    profile: Profile, // custom root path for config files etc.
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
      true => profile_config_dir(profile.clone())
        .map_err(|e| format!("Failed to get profile config dir: {}", e))?
        .join("custom"),
      false => conductor_config_dir(version, profile.clone())
        .map_err(|e| format!("Failed to get conductor config dir: {}", e))?,
    };

    let environment_path = match custom_binary_path.is_some() {
      true => profile_holochain_data_dir(profile.clone())
        .map_err(|e| format!("Failed to get profile's holochain data dir: {}", e))?
        .join("custom"),
      false => holochain_version_data_dir(version, profile.clone())
        .map_err(|e| format!("Failed to get profile's holochain version data dir: {}", e))?
      ,
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
      conductor_config_dir: conductor_config_path,
      environment_path,
      keystore_connection_url,
    };

    let version_str: String = version.into();

    let admin_window = self.app_handle.get_window("admin").unwrap();

    let state = match WebAppManager::launch(version, config, password).await {
      Ok(mut manager) => match version.eq(&HolochainVersion::default()) {
        true => match install_default_apps_if_necessary(&mut manager, admin_window).await {
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

  /// Gets a mutable reference to the `WebAppManager` for a given holochain version
  /// or custom binary if the `WebAppManager` already exists. Otherwise it launches
  /// a corresponding `HolochainManager` and `WebAppManager`.
  pub async fn get_or_launch_holochain(
    &mut self,
    holochain_id: HolochainId,
    profile: String,
  ) -> Result<&mut WebAppManager, String> {
    match holochain_id {
      HolochainId::HolochainVersion(version) => {
        if let None = self.holochain_managers.get(&version) {
          self.launch_holochain_manager(version.clone(), None, profile).await?;
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
            .launch_holochain_manager(HolochainVersion::custom(), Some(path), profile)
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
        .ok_or(format!("Holochain version {} is not running.", version.to_string())),
      HolochainId::CustomBinary => self.custom_binary_manager.as_mut().ok_or(String::from(
        "There is no Holochain running with custom binary",
      )),
    }?;

    match manager_state {
      RunningState::Running(m) => Ok(m),
      RunningState::Error(error) => {
        match holochain_id {
          HolochainId::HolochainVersion(version) =>
            Err(format!(
            "Holochain Version {} threw an exception: {:?}",
            version.to_string(),
            error
          )),
          HolochainId::CustomBinary => Err(format!("Custom holochain binary threw an exception: {:?}", error))
        }
      }
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
      if let Err(err) = w.unminimize() {
        log::error!("Error unminimizing the window: {:?}", err);
      }
      if let Err(err) = w.set_focus() {
        log::error!("Error setting focus to the window: {:?}", err);
      }
      return Ok(());
    }

    let manager = self.get_web_happ_manager(holochain_id)?;

    let index_path = manager.get_ui_index_path(app_id);
    let assets_path = manager.get_app_ui_dir(app_id);
    let local_storage_path = manager.get_app_local_storage_dir(app_id);
    let app_port = manager.holochain_manager.app_interface_port();
    let admin_port = manager.holochain_manager.admin_interface_port();
    let window_width = 1000.0;
    let window_height = 700.0;

    let window_builder = happ_window_builder(
      &self.app_handle,
      app_id.into(),
      window_label.clone(),
      app_id.into(),
      index_path,
      assets_path,
      local_storage_path,
      app_port,
      admin_port,
      window_width,
      window_height,
    );

    // placeholder for when apps come shipped with their custom icons:
    //
    // window_builder
    //  .icon(tauri::Icon::File(icon_path))
    //  .map_err(|err| format!("Error adding icon: {:?}", err))?


    if cfg!(target_os = "macos") {
      window_builder.build().map_err(|err| format!("Error opening app: {:?}", err))?;
    } else {
      let window = window_builder
        .menu(Menu::new().add_submenu(Submenu::new( // This overwrites the global menu on macOS (https://github.com/tauri-apps/tauri/issues/5768)
        "Settings",
        Menu::new().add_item(CustomMenuItem::new("show-devtools", "Show DevTools")),
         )))
        .build()
        .map_err(|err| format!("Error opening app: {:?}", err))?;
      // Listen to "open-devtools" command
      let a = self.app_handle.clone();
      let l = window_label.clone();
      window.on_menu_event(move |_| {
        if let Some(w) = a.get_window(l.as_str()) {
          w.open_devtools();
        }
      });
    }

    Ok(())
  }

}
