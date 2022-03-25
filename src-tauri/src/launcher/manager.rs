use holochain_manager::app_manager::AppManager;
use holochain_manager::config::LaunchHolochainConfig;
use holochain_web_app_manager::error::LaunchWebAppManagerError;
use holochain_web_app_manager::running_apps::RunningApps;
use std::{collections::HashMap, fs, process};
use tauri::Manager;
use tauri::{window::WindowBuilder, AppHandle, WindowUrl, Wry};
use url::Url;

use std::path::Path;

use holochain_manager::versions::HolochainVersion;
use holochain_web_app_manager::WebAppManager;

use crate::file_system::{
  conductor_config_path, data_path_for_holochain_version, keystore_data_path, pid_file_path,
};
use crate::{running_state::RunningState, system_tray::update_system_tray};

use super::config::LauncherConfig;

pub struct LauncherManager {
  config: LauncherConfig,
  pub holochain_managers:
    HashMap<HolochainVersion, RunningState<WebAppManager, LaunchWebAppManagerError>>,
}

impl LauncherManager {
  pub async fn launch(
    launcher_config: LauncherConfig,
    app_handle: &AppHandle<Wry>,
  ) -> Result<Self, String> {
    let versions = HolochainVersion::supported_versions();

    let mut manager = LauncherManager {
      holochain_managers: HashMap::new(),
      config: launcher_config,
    };

    for version in versions {
      manager.launch_holochain_manager(version, app_handle).await;
    }

    LauncherManager::write_pid_file()?;

    Ok(manager)
  }

  async fn launch_holochain_manager(
    &mut self,
    version: HolochainVersion,
    app_handle: &AppHandle<Wry>,
  ) -> () {
    let admin_port = portpicker::pick_unused_port().expect("No ports free");

    let conductor_config_path = conductor_config_path(version);
    let environment_path = data_path_for_holochain_version(version);
    let keystore_path = keystore_data_path(version.lair_keystore_version());

    let config = LaunchHolochainConfig {
      log_level: self.config.log_level,
      admin_port,
      conductor_config_path,
      environment_path,
      keystore_path,
    };

    let state = match WebAppManager::launch(version, config).await {
      Ok(mut manager) => {
        manager.on_running_apps_changed(move |_| {
          app_handle.emit_all("running_apps_changed", ());
        });
        RunningState::Running(manager)
      }
      Err(error) => RunningState::Error(error),
    };

    self.holochain_managers.insert(version, state);
  }

  pub fn get_web_happ_manager(
    &mut self,
    holochain_version: HolochainVersion,
  ) -> Result<&mut WebAppManager, String> {
    let manager_state = self
      .holochain_managers
      .get(&holochain_version)
      .ok_or(String::from("This holochain version is not running"))?;

    match manager_state {
      RunningState::Running(mut m) => Ok(&mut m),
      RunningState::Error(error) => Err(format!(
        "This holochain version is not running: {:?}",
        error
      )),
    }
  }

  /// Connects to the conductor, requests the list of running apps, updates the caddyfile and the system tray
  pub async fn on_apps_changed(&mut self, app_handle: &AppHandle<Wry>) -> Result<(), String> {
    let mut running_apps_by_version: HashMap<HolochainVersion, RunningApps> = HashMap::new();

    let versions: Vec<HolochainVersion> = self.holochain_managers.keys().cloned().collect();

    for version in versions {
      if let Ok(manager) = self.get_web_happ_manager(version.clone()) {
        let running_apps = manager.get_running_apps().await?;

        running_apps_by_version.insert(version.clone(), running_apps);
      }
    }

    update_system_tray(app_handle, &running_apps_by_version);

    // Iterate over the open windows, close any that has been uninstalled/disabled

    Ok(())
  }

  pub fn open_app(
    &self,
    holochain_version: HolochainVersion,
    app_id: &String,
    app_handle: &AppHandle<Wry>,
  ) -> Result<(), String> {
    // Iterate over the open windows, focus if the app is already open

    let manager = self.get_web_happ_manager(holochain_version)?;
    let port = manager
      .get_allocated_port(app_id)
      .ok_or(String::from("This app has no port attached"))?;

    WindowBuilder::new(
      app_handle,
      app_id.clone(),
      WindowUrl::External(Url::parse(format!("http://localhost:{}", port).as_str()).unwrap()),
    )
    .inner_size(1000.0, 700.0)
    .title(app_id)
    .build()
    .map_err(|err| format!("Error opening app: {:?}", err))?;

    Ok(())
  }

  fn write_pid_file() -> Result<(), String> {
    let pid = process::id();

    fs::write(pid_file_path(), format!("{}", pid))
      .map_err(|err| format!("Error writing the pid file: {:?}", err))?;

    Ok(())
  }

  pub fn remove_pid_file() -> Result<(), String> {
    fs::remove_file(&pid_file_path())
      .map_err(|err| format!("Error removing the pid file: {:?}", err))?;

    Ok(())
  }

  pub fn is_launcher_already_running() -> bool {
    Path::new(&pid_file_path()).exists()
  }
}
