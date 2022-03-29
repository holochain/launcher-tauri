use lair_keystore_manager::error::LaunchTauriSidecarError;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LaunchHolochainError {
  LaunchTauriSidecarError(LaunchTauriSidecarError),
  ErrorWritingPassword(String),
  CouldNotConnectToConductor(String),
}
