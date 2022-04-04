use holochain_manager::error::LaunchHolochainError;
use lair_keystore_manager::error::{LaunchChildError, FileSystemError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Serialize, Deserialize, Debug, Clone)]
pub enum LaunchWebAppManagerError {
  #[error("Error launching Holochain: `{0}`")]
  LaunchHolochainError(#[from] LaunchHolochainError),
  #[error("Error launching Caddy: `{0}`")]
  LaunchCaddyError(#[from] LaunchChildError),
  #[error("Failed to read or write from the filesystem: `{0}`")]
  FileSystemError(#[from] FileSystemError),
  #[error("Could not get a free application port: `{0}`")]
  CouldNotGetAppPort(String),
  #[error("Error launching the WebAppManager: `{0}`")]
  Other(String),
}
