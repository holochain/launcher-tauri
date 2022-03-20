use std::{fs, process};
use tauri::{AppHandle, Wry};

use crate::{connection_status::ConnectionStatus, system_tray::update_system_tray};
use std::path::Path;

use super::{
  caddy::CaddyManager,
  holochain::{
    conductor::{versions::v0_0_130::ConductorManagerV0_0_130, ConductorManager}, config::ManagerConfig,
    HolochainManager,
  },
};
use crate::managers::file_system::FileSystemManager;

pub struct LauncherManager {
  pub holochain_manager: ConnectionStatus<HolochainManager<ConductorManagerV0_0_130>>,
  pub caddy_manager: CaddyManager,
}

impl LauncherManager {
  pub async fn launch() -> Result<Self, String> {
    let holochain_manager =
      match HolochainManager::<ConductorManagerV0_0_130>::launch(ManagerConfig::default()).await {
        Ok(manager) => ConnectionStatus::Connected(manager),
        Err(error) => ConnectionStatus::Error { error },
      };
    let caddy_manager = CaddyManager::launch().await?;

    LauncherManager::write_pid_file()?;

    Ok(LauncherManager {
      holochain_manager,
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
    let running_apps = self
      .get_holochain_manager()?
      .conductor_manager
      .list_running_apps()
      .await?;

    self.refresh_caddyfile(&running_apps)?;

    update_system_tray(app_handle, &running_apps);

    Ok(())
  }

  /// Rewrites the Caddyfile with the appropriate port mapping
  fn refresh_caddyfile(&mut self, running_apps: &Vec<String>) -> Result<(), String> {
    log::info!("Refreshing caddyfile");

    let caddyfile = self
      .get_holochain_manager()?
      .ui_manager
      .caddy_config_for_apps(running_apps)?;

    self.caddy_manager.write_caddyfile(caddyfile)?;
    CaddyManager::reload_caddy()?;

    Ok(())
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
