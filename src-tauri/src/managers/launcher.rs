use tauri::{AppHandle, Wry};

use crate::system_tray::update_system_tray;

use super::{
  caddy::CaddyManager,
  holochain::{conductor::versions::V0_0_130::ConductorManagerV0_0_130, HolochainManager},
};

pub struct LauncherManager {
  holochain_manager: HolochainManager<ConductorManagerV0_0_130>,
  caddy_manager: CaddyManager,
}

impl LauncherManager {
  pub async fn launch() -> Result<Self, String> {
    let holochain_manager =
      HolochainManager::<ConductorManagerV0_0_130>::launch(ManagerConfig::default()).await?;
    let caddy_manager = CaddyManager::launch().await?;

    Ok(LauncherManager {
      holochain_manager,
      caddy_manager,
    })
  }

  /// Connects to the conductor, requests the list of running apps, updates the caddyfile and the system tray
  pub async fn on_apps_changed(&self, app_handle: &AppHandle<Wry>) -> Result<(), String> {
    let running_apps = self
      .holochain_manager
      .conductor_manager
      .list_running_apps()
      .await?;

    self.refresh_caddyfile(running_apps)?;

    update_system_tray(app_handle, running_apps);

    Ok(())
  }

  /// Rewrites the Caddyfile with the appropriate port mapping
  fn refresh_caddyfile(&self, running_apps: Vec<String>) -> Result<(), String> {
    log::info!("Refreshing caddyfile");

    let caddyfile = self
      .holochain_manager
      .ui_manager
      .caddy_config_for_apps(running_apps);

    self.caddy_manager.write_caddyfile(caddyfile)?;
    CaddyManager::reload_caddy()?;

    Ok(())
  }
}
