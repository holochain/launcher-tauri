use std::{fs, process, collections::HashMap};
use tauri::{AppHandle, Wry};

use std::path::Path;

use holochain_manager::{HolochainManager, versions::{HolochainVersion, launch_holochain}, error::LaunchHolochainError};

use crate::{running_state::RunningState, system_tray::update_system_tray, running_apps::RunningApps};
use crate::file_system::{conductor_config_path, keystore_data_path, conductor_data_path};

pub struct LauncherManager {
  pub holochain_managers: HashMap<HolochainVersion, RunningState<Box<dyn HolochainManager>, LaunchHolochainError>>,
  pub caddy_manager: CaddyManager,
}

impl LauncherManager {
  pub async fn launch(log_level: log::Level) -> Result<Self, String> {
    let versions = HolochainVersion::supported_versions();

    let mut holochain_managers: HashMap<HolochainVersion, Box<dyn HolochainManager>> = HashMap::new();

    for version in versions {
      let admin_port = portpicker::pick_unused_port().expect("No ports free");

      let config_path = conductor_config_path(version);
      let environment_path = conductor_data_path(version);
      let keystore_path = keystore_data_path(version.lair_keystore_version());

      let state = match launch_holochain(version, log_level, admin_port, config_path, environment_path, keystore_path).await {
        Ok(manager) => RunningState::Running(manager),
        Err(error) => RunningState::Error(error)
      };

      holochain_managers.insert(version, state);
    }
    
    let caddy_manager = CaddyManager::launch().await?;

    LauncherManager::write_pid_file()?;

    Ok(LauncherManager {
      holochain_managers,
      caddy_manager,
    })
  }

  pub fn get_holochain_manager(
    &mut self,
  ) -> Result<&mut HolochainManager<ConductorManagerV0_0_130>, String> {
    match &mut self.holochain_manager {
      ConnectionStatus::Connected(m) => Ok(m),
      ConnectionStatus::Error { error } => Err(error.clone()),
    }
  }

  /// Connects to the conductor, requests the list of running apps, updates the caddyfile and the system tray
  pub async fn on_apps_changed(&mut self, app_handle: &AppHandle<Wry>) -> Result<(), String> {
    let running_apps  = self.get_running_apps().await?;

    self.refresh_caddyfile(&running_apps)?;

    update_system_tray(app_handle, &running_apps);

    // Iterate over the open windows, close any that has been uninstalled/disabled

    Ok(())
  }

  pub fn open_app(&self, holochain_version: HolochainVersion, app_id: &String, app_handle: &AppHandle<Wry>) -> Result<(), String> {

    let free_port = portpicker::pick_unused_port().expect("No ports free");

    let caddyfile_path = FileSystemManager::caddyfile_path();
    let caddyfile_contents = 

    let port = port_mapping
      .get_ui_port_for_app(&self.holochain_version, &app_id)
      .ok_or(String::from("This app doesn't have a UI installed."))?;

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

  pub async fn get_running_apps(&self) -> Result<RunningApps, String> {
    let running_apps = self
      .get_holochain_manager()?
      .conductor_manager
      .list_running_apps()
      .await?;
  }

  fn write_pid_file() -> Result<(), String> {
    let pid = process::id();

    fs::write(FileSystemManager::pid_file_path(), format!("{}", pid))
      .map_err(|err| format!("Error writing the pid file: {:?}", err))?;

    Ok(())
  }

  pub fn remove_pid_file() -> Result<(), String> {
    fs::remove_file(&FileSystemManager::pid_file_path())
      .map_err(|err| format!("Error removing the pid file: {:?}", err))?;

    Ok(())
  }

  pub fn is_launcher_already_running() -> bool {
    Path::new(&FileSystemManager::pid_file_path()).exists()
  }
}
