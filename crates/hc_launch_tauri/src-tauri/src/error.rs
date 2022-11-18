use thiserror;

use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, Serialize, Deserialize, Clone)]
pub enum HcLaunchTauriError {
  #[error("Failed to get data to sign from unsigned zome call: \"{0}\"")]
  DataToSignError(String),
  #[error("Failed to sign by public key: \"{0}\"")]
  SignZomeCallError(String),
}

