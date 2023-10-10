use holochain_launcher_utils::window_builder::{happ_window_builder, UISource};
use holochain_manager::config::LaunchHolochainConfig;
use holochain_manager::errors::{InitializeConductorError, LaunchHolochainError};
use holochain_web_app_manager::{derive_window_label, error::LaunchWebAppManagerError};
use lair_keystore_manager::error::{LairKeystoreError, LaunchChildError};
use lair_keystore_manager::utils::create_dir_if_necessary;
use lair_keystore_manager::versions::v0_3::LairKeystoreManagerV0_3;
use lair_keystore_manager::LairKeystoreManager;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;
use tauri::api::process::Command;
use tauri::{AppHandle, Manager, PhysicalSize};
use tauri::{CustomMenuItem, Menu, Submenu};

use holochain_manager::versions::HolochainVersion;
use holochain_web_app_manager::WebAppManager;

use crate::file_system::{
  conductor_config_dir, holochain_version_data_dir, keystore_data_dir, launcher_config_dir,
  profile_config_dir, profile_holochain_data_dir, profile_lair_dir, Profile,
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

impl Into<String> for HolochainId {
  fn into(self) -> String {
    match self {
      HolochainId::HolochainVersion(version) => version.to_string(),
      HolochainId::CustomBinary => String::from("Custom Binary"),
    }
  }
}

pub struct LauncherManager {
  app_handle: Arc<AppHandle>,
  config: LauncherConfig,

  pub holochain_managers:
    HashMap<HolochainVersion, RunningState<WebAppManager, LaunchWebAppManagerError>>,
  pub custom_binary_manager: Option<RunningState<WebAppManager, LaunchWebAppManagerError>>,
  pub lair_keystore_manager: RunningState<Box<dyn LairKeystoreManager>, KeystoreStatus>,
  pub profile: String,
}

impl LauncherManager {
  pub async fn launch(app_handle: Arc<AppHandle>, profile: Profile) -> Result<Self, LauncherError> {
    create_dir_if_necessary(&profile_lair_dir(profile.clone())?)?;
    create_dir_if_necessary(&profile_holochain_data_dir(profile.clone())?)?;
    create_dir_if_necessary(&profile_config_dir(profile.clone())?)?;
    create_dir_if_necessary(&launcher_config_dir(profile.clone())?)?;

    let keystore_path = keystore_data_dir(
      LairKeystoreManagerV0_3::lair_keystore_version(),
      profile.clone(),
    )?;

    let is_initialized = LairKeystoreManagerV0_3::is_initialized(keystore_path);

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
  pub async fn initialize_keystore_and_launch(
    &mut self,
    password: String,
    profile: Profile,
  ) -> Result<(), String> {
    // emitting signal to the front-end for progress indication
    self
      .app_handle
      .get_window("admin")
      .unwrap()
      .emit("progress-update", String::from("Initializing keystore"))
      .map_err(|e| format!("Failed to send signal to the frontend: {:?}", e))?;

    let keystore_path = keystore_data_dir(
      LairKeystoreManagerV0_3::lair_keystore_version(),
      profile.clone(),
    )
    .map_err(|e| format!("Failed to get keystore data dir: {}", e))?;

    LairKeystoreManagerV0_3::initialize(keystore_path, password.clone())
      .await
      .map_err(|err| format!("Error initializing the keystore: {:?}", err))?;

    // emitting signal to the front-end for progress indication
    self
      .app_handle
      .get_window("admin")
      .unwrap()
      .emit("progress-update", String::from("Launching keystore"))
      .map_err(|e| format!("Failed to send signal to the frontend: {:?}", e))?;

    // sleep for 300ms to prevent potential issue with DevHub's public key missing in lair keystore (https://github.com/holochain/launcher/issues/146)
    std::thread::sleep(std::time::Duration::from_millis(300));

    self.launch_managers(password, profile).await?;

    Ok(())
  }

  /// Launches LairKeystoreManager, HolochainManager(s) and WebAppManager(s).
  pub async fn launch_managers(
    &mut self,
    password: String,
    profile: Profile,
  ) -> Result<(), String> {
    let keystore_path = keystore_data_dir(
      LairKeystoreManagerV0_3::lair_keystore_version(),
      profile.clone(),
    )
    .map_err(|e| format!("Failed to get keystore data dir: {}", e))?;

    let lair_keystore_manager =
      LairKeystoreManagerV0_3::launch(self.config.log_level, keystore_path, password.clone())
        .await
        .map_err(|err| format!("Error launching the keystore: {:?}", err))?;

    self.lair_keystore_manager = RunningState::Running(Box::new(lair_keystore_manager));

    // sleep for 300ms to prevent potential issue with DevHub's public key missing in lair keystore (https://github.com/holochain/launcher/issues/146)
    std::thread::sleep(std::time::Duration::from_millis(300));

    let mut holochain_versions_to_run = self.config.running_versions.clone();

    holochain_versions_to_run.insert(HolochainVersion::default());

    for version in holochain_versions_to_run {
      // emitting signal to the front-end for progress indication
      self
        .app_handle
        .get_window("admin")
        .unwrap()
        .emit(
          "progress-update",
          format!("Launching Holochain version {}", version.to_string()),
        )
        .map_err(|e| format!("Failed to send signal to the frontend: {:?}", e))?;

      self
        .launch_holochain_manager(version, None, profile.clone())
        .await?;
    }

    if let Some(path) = self.config.custom_binary_path.clone() {
      self
        .launch_holochain_manager(HolochainVersion::custom(), Some(path), profile.clone())
        .await?;
    } else {
      // If no custom holochain binary is specified in launcher-config.yaml, remove the data associated to previous
      // custom holochain binaries
      let _r = std::fs::remove_dir_all(
        profile_config_dir(profile.clone())
          .map_err(|e| format!("Failed to get profile config dir: {}", e))?
          .join("custom"),
      );
      let _r = std::fs::remove_dir_all(
        profile_holochain_data_dir(profile)
          .map_err(|e| format!("Failed to get profile holochain data dir: {}", e))?
          .join("custom"),
      );
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

    let admin_port = match option_env!("ADMIN_PORT") {
      Some(p) => p.parse().unwrap(),
      None => portpicker::pick_unused_port().expect("No ports free"),
    };

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
      false => holochain_version_data_dir(&version, profile.clone())
        .map_err(|e| format!("Failed to get profile's holochain version data dir: {}", e))?,
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
      bootstrap_server_url: self.config.bootstrap_server_url.clone(),
      signaling_server_url: self.config.signaling_server_url.clone(),
    };

    let version_str: String = version.into();

    let admin_window = self.app_handle.get_window("admin").unwrap();

    let state = match WebAppManager::launch(version, config, self.app_handle.clone(), password)
      .await
    {
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
          self
            .launch_holochain_manager(version.clone(), None, profile)
            .await?;
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
      HolochainId::HolochainVersion(version) => self.holochain_managers.get_mut(&version).ok_or(
        format!("Holochain version {} is not running.", version.to_string()),
      ),
      HolochainId::CustomBinary => self.custom_binary_manager.as_mut().ok_or(String::from(
        "There is no Holochain running with custom binary",
      )),
    }?;

    match manager_state {
      RunningState::Running(m) => Ok(m),
      RunningState::Error(error) => match holochain_id {
        HolochainId::HolochainVersion(version) => Err(format!(
          "Holochain Version {} threw an exception: {:?}",
          version.to_string(),
          error
        )),
        HolochainId::CustomBinary => Err(format!(
          "Custom holochain binary threw an exception: {:?}",
          error
        )),
      },
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
      if let Ok(web_app_manager) =
        self.get_web_happ_manager(HolochainId::HolochainVersion(version.clone()))
      {
        let running_apps = web_app_manager.list_apps().await?;

        all_installed_apps
          .by_version
          .insert(version.clone(), running_apps);
      }
    }

    if let Some(m) = &mut self.custom_binary_manager {
      match m.get_running() {
        Ok(web_app_manager) => {
          let running_apps = web_app_manager.list_apps().await?;

          all_installed_apps.custom_binary = Some(running_apps);
        }
        Err(_) => {}
      }
    }

    update_system_tray(&self.app_handle, &all_installed_apps);

    // TODO: Iterate over the open windows, close any that has been uninstalled/disabled

    Ok(())
  }

  pub fn open_app(&mut self, holochain_id: HolochainId, app_id: &String, visible: bool) -> Result<(), String> {
    let window_label = derive_window_label(&app_id);

    // Iterate over the open windows, focus if the app is already open

    if let Some(w) = self.app_handle.get_window(window_label.as_str()) {
      if visible {
        if let Err(err) = w.show() {
          log::error!("Error showing the window: {:?}", err);
        }
        if let Err(err) = w.unminimize() {
          log::error!("Error unminimizing the window: {:?}", err);
        }
        if let Err(err) = w.set_focus() {
          log::error!("Error setting focus to the window: {:?}", err);
        }
      }
      return Ok(());
    }

    let manager = self.get_web_happ_manager(holochain_id)?;

    // assuming at some point there may be multiple UI's for the same happ, open the "default" folder for now
    let ui_name = String::from("default");
    let assets_path = manager.get_app_assets_dir(app_id, &ui_name);
    let local_storage_path = manager.get_app_local_storage_dir(app_id, &ui_name);
    let app_port = manager.holochain_manager.app_interface_port();
    let admin_port = manager.holochain_manager.admin_interface_port();

    let mut window_builder = happ_window_builder(
      &self.app_handle,
      app_id.into(),
      window_label.clone(),
      app_id.into(),
      UISource::Path(assets_path),
      local_storage_path,
      app_port,
      admin_port,
      true,
    );

    if !visible {
      window_builder = window_builder.visible(false);
    }

    // needs to be removed in order for set_size() to work apparently
    // window_builder = window_builder.maximized(true);

    // set window size to 80% of a common screen resolution of 1920 x 1080.
    window_builder = window_builder.inner_size(1536.0, 864.0);

    // add launcher API scripts
    window_builder = window_builder.initialization_script(include_str!("../../../api_scripts/dist/launcher-api-scripts.js"));

    // placeholder for when apps come shipped with their custom icons:
    //
    // window_builder
    //  .icon(tauri::Icon::File(icon_path))
    //  .map_err(|err| format!("Error adding icon: {:?}", err))?

    // factor of the monitor size to which the window shall be scaled
    let _scaling_factor = 0.8;

    if cfg!(target_os = "macos") {
      let _window = window_builder
        .build()
        .map_err(|err| format!("Error opening app: {:?}", err))?;
      // removing this for now since it behaves inconsistently
      // set_window_size(window, scaling_factor);
    } else {
      window_builder = window_builder.menu(Menu::new().add_submenu(Submenu::new(
        // This overwrites the global menu on macOS (https://github.com/tauri-apps/tauri/issues/5768)
        "Settings",
        Menu::new().add_item(CustomMenuItem::new("show-devtools", "Show DevTools")),
      )));

      // Window opens weirdly out of bounds on windows if not centered.
      if cfg!(target_os = "windows") {
        window_builder = window_builder.center();
      }

      let window = window_builder
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

      // removing this for now because it behaves inconsistently
      // set_window_size(window, scaling_factor);
    }

    Ok(())
  }
}

fn _set_window_size(window: tauri::window::Window, scaling_factor: f64) -> () {
  // set window to 80% of the monitor size if possible
  match window.current_monitor() {
    Ok(maybe_monitor) => {
      if let Some(monitor) = maybe_monitor {
        let size = monitor.size();
        let new_width = (scaling_factor * size.width as f64) as u32;
        let new_height = (scaling_factor * size.height as f64) as u32;

        let new_size = PhysicalSize::new(new_width, new_height);

        match window.set_size(new_size) {
          Ok(()) => (),
          Err(e) => log::error!("Failed to set window size: {:?}", e),
        };
      }
    }
    Err(e) => log::error!("Failed to get monitor option: {:?}", e),
  };
}
