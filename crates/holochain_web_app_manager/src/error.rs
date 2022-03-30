use holochain_manager::error::LaunchHolochainError;
use lair_keystore_manager::error::LaunchTauriSidecarError;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LaunchWebAppManagerError {
  LaunchHolochainError(LaunchHolochainError),
  LaunchCaddyError(LaunchTauriSidecarError),
  CouldNotGetAppPort(String),
  Other(String),
}
