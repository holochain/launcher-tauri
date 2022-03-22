use std::collections::HashMap;

use crate::holochain_version::holochain_types_latest::{
  prelude::SerializedBytes, web_app::WebAppBundle,
};
use crate::holochain_version::HolochainVersion;
use crate::running_state::RunningState;

use self::conductor::ConductorManager;
use self::config::ManagerConfig;

use super::uis::UiManager;

pub mod conductor;
pub mod config;
mod lair_keystore;

pub struct HolochainManager<CM: ConductorManager> {
  _config: ManagerConfig,

  pub admin_port: u16,
  pub conductor_manager: CM,
  pub ui_manager: UiManager,
}

pub type HolochainState<CM: ConductorManager> = RunningState<HolochainManager<CM>, RunHolochainError>;

pub enum RunHolochainError {}

pub struct HolochainStateInfo {
  admin_port: u16,
  holochain_version: HolochainVersion,
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
      admin_port,
      ui_manager,
    })
  }

}
