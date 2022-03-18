use std::collections::HashMap;

use crate::holochain_version::holochain_types_latest::{
  prelude::SerializedBytes, web_app::WebAppBundle,
};
use crate::holochain_version::HolochainVersion;

use self::conductor::ConductorManager;
use self::config::ManagerConfig;

use super::uis::UiManager;

pub mod config;
pub mod conductor;

pub struct HolochainManager<CM: ConductorManager> {
  config: ManagerConfig,

  pub conductor_manager: CM,
  pub ui_manager: UiManager,
}

impl<CM: ConductorManager + std::marker::Send> HolochainManager<CM> {
  pub async fn launch(config: ManagerConfig) -> Result<Self, String> {
    let admin_port = portpicker::pick_unused_port().expect("No ports free");

    let conductor_manager = CM::launch_holochain(config.log_level, admin_port).await?;

    Ok(HolochainManager {
      config,
      conductor_manager,
      ui_manager: UiManager::new(HolochainVersion::V0_0_130),
    })
  }

  pub async fn install_app(
    &self,
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

  async fn handle_on_apps_changed() -> Result<(), String> {}

}
