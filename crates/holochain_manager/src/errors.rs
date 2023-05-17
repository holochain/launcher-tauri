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
  #[error("Could not initialize conductor: `{0}`")]
  CouldNotInitializeConductor(#[from] InitializeConductorError),
  #[error("Failed to overwrite config: `{0}`")]
  FailedToOverwriteConfig(String),
  #[error("Impossible error: `{0}`")]
  ImpossibleError(String),
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



#[derive(Error, Serialize, Deserialize, Debug, Clone)]
pub enum InitializeConductorError {
  #[error("Unknown Error: `{0}`")]
  UnknownError(String),
  #[error("Could not connect to the database of the conductor: `{0}`")]
  SqliteError(String),
  #[error("Address already in use: `{0}`")]
  AddressAlreadyInUse(String),
}
