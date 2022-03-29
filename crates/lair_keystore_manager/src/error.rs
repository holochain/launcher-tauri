use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content")]
pub enum LairKeystoreError {
  LaunchTauriSidecarError(LaunchTauriSidecarError),
  ErrorWritingPassword(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "content")]
pub enum LaunchTauriSidecarError {
  BinaryNotFound,
  FailedToExecute(String),
}
