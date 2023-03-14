use std::{fmt, io};
use thiserror::Error;

use serde::{Deserialize, Serialize};

#[derive(Error, Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content")]
pub enum LairKeystoreError {
  #[error("Failed to launch child: `{0}`")]
  LaunchChildError(#[from] LaunchChildError),
  #[error("Failed to write the password: `{0}`")]
  ErrorWritingPassword(String),
  #[error("Incorrect password")]
  IncorrectPassword,
  #[error("Failed to create LairClient: `{0}`")]
  ErrorCreatingLairClient(String),
  #[error("Failed to create temp dir: `{0}`")]
  ErrorReadingLairConfig(String),
  #[error("Failed to read lair-keysstore-config.yaml: `{0}`")]
  ErrorWritingLairConfig(String),
  #[error("Failed to write lair-keysstore-config.yaml: `{0}`")]
  ErrorCreatingSimLink(String),
  #[error("Lair Keystore Error: `{0}`")]
  OtherError(String),
  #[error("Failed to sign zome call: `{0}`")]
  SignZomeCallError(String),
}

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "content")]
pub enum LaunchChildError {
  #[error("Sidecar binary was not found")]
  BinaryNotFound,
  #[error("Failed to execute sidecar binary: `{0}`")]
  FailedToExecute(String),
  #[error("Failed to read or write from the filesystem: `{0}`")]
  FileSystemError(#[from] FileSystemError),
}

#[derive(Error, Debug, Serialize, Deserialize, Clone)]
pub struct FileSystemError(String);

impl fmt::Display for FileSystemError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    // Use `self.number` to refer to each positional data point.
    write!(f, "{}", self.0)
  }
}

impl From<io::Error> for FileSystemError {
  fn from(err: io::Error) -> Self {
    FileSystemError(format!("{:?}", err))
  }
}

impl From<FileSystemError> for LairKeystoreError {
  fn from(err: FileSystemError) -> Self {
    LairKeystoreError::LaunchChildError(LaunchChildError::FileSystemError(err))
  }
}
