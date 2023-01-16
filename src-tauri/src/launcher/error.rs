use serde::{Deserialize, Serialize};
use thiserror::Error;
use lair_keystore_manager::error::FileSystemError;

#[derive(Error, Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content")]
pub enum LauncherError {
  #[error("There is already another instance of the Holochain Launcher running")]
  AnotherInstanceIsAlreadyRunning,
  #[error("Failed to read or write from the filesystem: `{0}`")]
  FileSystemError(#[from] FileSystemError),
  #[error("There are still files from previous versions of the Holochain Launcher")]
  OldFilesExist,
  #[error("There was an error with the launcher configuration: `{0}`")]
  ConfigError(String),
  #[error("Error Launching: `{0}`")]
  ErrorLaunching(String),
  #[error("Unauthorized: `{0}`")]
  Unauthorized(String),
  #[error("Failed to get system directory: `{0}`")]
  SystemDirError(String)
}
