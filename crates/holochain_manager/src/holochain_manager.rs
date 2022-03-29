use async_trait::async_trait;
use holochain_client_0_0_130::InstalledAppInfo;
use lair_keystore_manager::versions::LairKeystoreVersion;

use crate::app_manager::AppManager;
use crate::config::LaunchHolochainConfig;
use crate::error::LaunchHolochainError;
use crate::versions::HolochainVersion;

#[async_trait]
pub trait HolochainManager:
  Send + Sync + AppManager<InstalledApps = Vec<InstalledAppInfo>>
{
  /// Launch the conductor
  /// If there already was an instantiated conductor in the given paths, use those folders
  /// If not, create the necessary files and folder to start afresh
  /// 
  /// This assumes that lair_keystore is already running in the configured path
  async fn launch(config: LaunchHolochainConfig, password: String) -> Result<Self, LaunchHolochainError>
  where
    Self: Sized;

  fn holochain_version() -> HolochainVersion
  where
    Self: Sized;

  fn lair_keystore_version(&self) -> LairKeystoreVersion;

  async fn get_app_interface_port(&mut self) -> Result<u16, String>;
}
