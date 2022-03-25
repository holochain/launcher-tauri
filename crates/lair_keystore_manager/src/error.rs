use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum LaunchTauriSidecarError {
  BinaryNotFound,
  FailedToExecute(String),
}
