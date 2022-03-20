use std::collections::HashMap;

use crate::holochain_version::holochain_types_latest::{
  prelude::SerializedBytes, web_app::WebAppBundle,
};

use self::conductor::ConductorManager;
use self::config::ManagerConfig;

use super::uis::UiManager;

pub mod conductor;
pub mod config;

pub struct HolochainManager<CM: ConductorManager> {
  _config: ManagerConfig,

  pub conductor_manager: CM,
  pub ui_manager: UiManager,
}

impl<CM: ConductorManager + std::marker::Send> HolochainManager<CM> {
  pub async fn launch(config: ManagerConfig) -> Result<Self, String> {
    let admin_port = portpicker::pick_unused_port().expect("No ports free");

    let mut conductor_manager = CM::launch_holochain(config.log_level, admin_port).await?;

    let app_port = conductor_manager.get_app_port().await?;

    let ui_manager = UiManager::launch(CM::holochain_version(), admin_port, app_port)?;

    Ok(HolochainManager {
      _config: config,
      conductor_manager,
      ui_manager,
    })
  }

  pub async fn install_app(
    &mut self,
    app_id: String,
    web_app_bundle: WebAppBundle,
    uid: Option<String>,
    membrane_proofs: HashMap<String, SerializedBytes>,
  ) -> Result<(), String> {
    let app_bundle = web_app_bundle
      .happ_bundle()
      .await
      .or(Err("Failed to resolve hApp bundle"))?;

    // Install app in conductor manager
    self
      .conductor_manager
      .install_app(&app_id, app_bundle, uid, membrane_proofs)
      .await
      .map_err(|err| {
        log::error!("Error installing hApp in the conductor: {}", err);
        err
      })?;

    // Install app in UI manager

    let web_ui_zip_bytes = web_app_bundle
      .web_ui_zip_bytes()
      .await
      .or(Err("Failed to resolve Web UI"))?;

    self
      .ui_manager
      .install_app_ui(&app_id, web_ui_zip_bytes.to_vec())
      .await
      .map_err(|err| {
        log::error!("Error installing the UI for hApp: {}", err);
        err
      })?;

    Ok(())
  }

  pub async fn uninstall_app(&mut self, app_id: String) -> Result<(), String> {
    // Uninstall app in conductor manager
    self
      .conductor_manager
      .uninstall_app(&app_id)
      .await
      .map_err(|err| {
        log::error!("Error uninstalling hApp in the conductor: {}", err);
        err
      })?;

    self.ui_manager.uninstall_app_ui(&app_id).await?;

    Ok(())
  }
}
