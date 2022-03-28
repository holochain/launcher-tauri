use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum LairKeystoreError {
  LaunchTauriSidecarError(LaunchTauriSidecarError),
  ErrorWritingPassword(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LaunchTauriSidecarError {
  BinaryNotFound,
  FailedToExecute(String),
}
