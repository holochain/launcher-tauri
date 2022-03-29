use holochain_manager::app_manager::AppManager;
use holochain_manager::config::LaunchHolochainConfig;
use holochain_web_app_manager::error::LaunchWebAppManagerError;
use holochain_web_app_manager::installed_web_app_info::InstalledWebAppInfo;
use lair_keystore_manager::error::LairKeystoreError;
use lair_keystore_manager::utils::create_dir_if_necessary;
use lair_keystore_manager::versions::v0_1_0::LairKeystoreManagerV0_1_0;
use lair_keystore_manager::LairKeystoreManager;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::{collections::HashMap, fs, process};
use sysinfo::{ProcessExt, System, SystemExt};
use tauri::{window::WindowBuilder, AppHandle, Manager, WindowUrl};
use url::Url;

use std::path::Path;

use holochain_manager::versions::HolochainVersion;
use holochain_web_app_manager::WebAppManager;

use crate::file_system::{
  data_path_for_holochain_version, keystore_data_path, root_config_path,
  root_data_path, root_lair_path, config_environment_path,
};
use crate::{running_state::RunningState, system_tray::update_system_tray, LauncherState};

use super::config::LauncherConfig;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content")]
pub enum KeystoreStatus {
  InitNecessary,
  PasswordNecessary,
  LaunchKeystoreError(LairKeystoreError),
}

pub struct LauncherManager {
  app_handle: AppHandle,
  config: LauncherConfig,
  pub holochain_managers:
    HashMap<HolochainVersion, RunningState<WebAppManager, LaunchWebAppManagerError>>,
  pub lair_keystore_manager: RunningState<Box<dyn LairKeystoreManager>, KeystoreStatus>,
}

impl LauncherManager {
  pub async fn launch(
    launcher_config: LauncherConfig,
    app_handle: AppHandle,
  ) -> Result<Self, String> {
    create_dir_if_necessary(&root_lair_path());
    create_dir_if_necessary(&root_data_path());
    create_dir_if_necessary(&root_config_path());

    let keystore_path = keystore_data_path(LairKeystoreManagerV0_1_0::lair_keystore_version());

    let is_initialized = LairKeystoreManagerV0_1_0::is_initialized(keystore_path);

    let keystore_status = match is_initialized {
      true => KeystoreStatus::PasswordNecessary,
      false => KeystoreStatus::InitNecessary,
    };

    let app_handle2 = app_handle.clone();
    let manager = LauncherManager {
      app_handle: app_handle.clone(),
      holochain_managers: HashMap::new(),
      config: launcher_config,
      lair_keystore_manager: RunningState::Error(keystore_status),
    };

    app_handle.listen_global("running_apps_changed", move |_| {
      let launcher_state: &LauncherState = &app_handle2.state();
      tauri::async_runtime::block_on(async move {
        if let Err(err) = launcher_state
          .get_launcher_manager()
          .unwrap()
          .lock()
          .await
          .on_apps_changed()
          .await
        {
          log::error!("Couldn't refresh apps: {:?}", err)
        }
      });
    });

    Ok(manager)
  }

  pub async fn initialize_and_launch_keystore(&mut self, password: String) -> Result<(), String> {
    let keystore_path = keystore_data_path(LairKeystoreManagerV0_1_0::lair_keystore_version());

    LairKeystoreManagerV0_1_0::initialize(keystore_path, password.clone())
      .map_err(|err| format!("Error initializing the keystore: {:?}", err))?;

    std::thread::sleep(Duration::from_millis(3000));

    self.launch_keystore(password).await?;

    Ok(())
  }

  pub async fn launch_keystore(&mut self, password: String) -> Result<(), String> {
    let keystore_path = keystore_data_path(LairKeystoreManagerV0_1_0::lair_keystore_version());
    let lair_keystore_manager =
      LairKeystoreManagerV0_1_0::launch(self.config.log_level, keystore_path, password.clone())
        .map_err(|err| format!("Error launching the keystore: {:?}", err))?;

    self.lair_keystore_manager = RunningState::Running(Box::new(lair_keystore_manager));

    for version in HolochainVersion::supported_versions() {
      self
        .launch_holochain_manager(version, password.clone())
        .await?;
    }

    Ok(())
  }

  pub async fn launch_holochain_manager(
    &mut self,
    version: HolochainVersion,
    password: String,
  ) -> Result<(), String> {
    let admin_port = portpicker::pick_unused_port().expect("No ports free");

    let conductor_config_path = config_environment_path(version);
    let environment_path = data_path_for_holochain_version(version);

    let keystore_connection_url = self.get_lair_keystore_manager()?.connection_url();

    let config = LaunchHolochainConfig {
      log_level: self.config.log_level,
      admin_port,
      config_environment_path: conductor_config_path,
      environment_path,
      keystore_connection_url,
    };

    let state =
      match WebAppManager::launch(version, config, password, self.app_handle.clone()).await {
        Ok(manager) => RunningState::Running(manager),
        Err(error) => RunningState::Error(error),
      };

    self.holochain_managers.insert(version, state);

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

  pub fn get_web_happ_manager(
    &mut self,
    holochain_version: HolochainVersion,
  ) -> Result<&mut WebAppManager, String> {
    let manager_state = self
      .holochain_managers
      .get_mut(&holochain_version)
      .ok_or(String::from("This holochain version is not running"))?;

    match manager_state {
      RunningState::Running(m) => Ok(m),
      RunningState::Error(error) => Err(format!(
        "This holochain version is not running: {:?}",
        error
      )),
    }
  }

  /// Connects to the conductor, requests the list of running apps, updates the caddyfile and the system tray
  pub async fn on_apps_changed(&mut self) -> Result<(), String> {
    let mut running_apps_by_version: HashMap<HolochainVersion, Vec<InstalledWebAppInfo>> =
      HashMap::new();

    let versions: Vec<HolochainVersion> = self.holochain_managers.keys().cloned().collect();

    for version in versions {
      if let Ok(manager) = self.get_web_happ_manager(version.clone()) {
        let running_apps = manager.list_apps().await?;

        running_apps_by_version.insert(version.clone(), running_apps);
      }
    }

    update_system_tray(&self.app_handle, &running_apps_by_version);

    // Iterate over the open windows, close any that has been uninstalled/disabled

    Ok(())
  }

  pub fn open_app(
    &mut self,
    holochain_version: HolochainVersion,
    app_id: &String,
  ) -> Result<(), String> {
    // Iterate over the open windows, focus if the app is already open

    let manager = self.get_web_happ_manager(holochain_version)?;
    let port = manager
      .get_allocated_port(app_id)
      .ok_or(String::from("This app has no port attached"))?;

    WindowBuilder::new(
      &self.app_handle,
      app_id.clone(),
      WindowUrl::External(Url::parse(format!("http://localhost:{}", port).as_str()).unwrap()),
    )
    .inner_size(1000.0, 700.0)
    .title(app_id)
    .build()
    .map_err(|err| format!("Error opening app: {:?}", err))?;

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