use std::io;

use lair_keystore_manager::error::{FileSystemError, LaunchChildError};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Serialize, Deserialize, Debug, Clone)]
pub enum LaunchHolochainError {
  #[error("Failed to launch child: `{0}`")]
  LaunchChildError(#[from] LaunchChildError),
  #[error("Failed to write the password: `{0}`")]
  ErrorWritingPassword(String),
  #[error("Error with the filesystem: `{0}`")]
  IoError(String),
  #[error("Could not connect to the conductor: `{0}`")]
  CouldNotConnectToConductor(String),
  #[error("Could not connect to the database of the conductor: `{0}`")]
  SqliteError(String),
}

impl From<FileSystemError> for LaunchHolochainError {
  fn from(err: FileSystemError) -> Self {
    LaunchHolochainError::LaunchChildError(LaunchChildError::FileSystemError(err))
  }
}

impl From<io::Error> for LaunchHolochainError {
  fn from(err: io::Error) -> Self {
    LaunchHolochainError::IoError(format!("{:?}", err))
  }
}
