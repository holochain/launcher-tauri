use lair_keystore_manager::error::{LaunchChildError, FileSystemError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Serialize, Deserialize, Debug, Clone)]
pub enum LaunchHolochainError {
  #[error("Failed to launch child: `{0}`")]
  LaunchChildError(#[from] LaunchChildError),
  #[error("Failed to write the password: `{0}`")]
  ErrorWritingPassword(String),
  #[error("Could not connect to the conductor: `{0}`")]
  CouldNotConnectToConductor(String),
}

impl From<FileSystemError> for LaunchHolochainError {
  fn from(err: FileSystemError) -> Self {
    LaunchHolochainError::LaunchChildError(LaunchChildError::FileSystemError(err))
  }
}
