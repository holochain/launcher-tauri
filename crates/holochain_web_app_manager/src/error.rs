use holochain_manager::errors::LaunchHolochainError;
use lair_keystore_manager::error::FileSystemError;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Serialize, Deserialize, Debug, Clone)]
pub enum LaunchWebAppManagerError {
  #[error("Error launching Holochain: `{0}`")]
  LaunchHolochainError(#[from] LaunchHolochainError),
  #[error("Failed to read or write from the filesystem: `{0}`")]
  FileSystemError(#[from] FileSystemError),
  #[error("Error launching the WebAppManager: `{0}`")]
  Other(String),
}
