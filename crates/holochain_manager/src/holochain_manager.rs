
use async_trait::async_trait;

use crate::config::LaunchHolochainConfig;
use crate::error::LaunchHolochainError;
use crate::versions::HolochainVersion;
use crate::app_manager::AppManager;

#[async_trait]
pub trait HolochainManager: Send + Sync + AppManager<RunningApps = Vec<String>> {
  /// Launch the conductor
  /// If there already was an instantiated conductor in the given paths, use those folders
  /// If not, create the necessary files and folder to start afresh
  async fn launch(config: LaunchHolochainConfig) -> Result<Self, LaunchHolochainError>
  where
    Self: Sized;

  fn holochain_version() -> HolochainVersion
  where
    Self: Sized;

  async fn get_app_interface_port(&mut self) -> Result<u16, String>;
}
