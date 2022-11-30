use thiserror;

use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, Serialize, Deserialize, Clone)]
pub enum HcLaunchTauriError {
  #[error("Failed to get data to sign from unsigned zome call: \"{0}\"")]
  DataToSignError(String),
  #[error("Failed to sign by public key: \"{0}\"")]
  SignZomeCallError(String),
}


#[derive(Debug, thiserror::Error)]
pub enum HcLaunchError {

    #[error("Specified UI path \"{0}\" does not exist.")]
    UiPathDoesNotExist(String),

    /// anything else
    #[error("Unknown error: {0}")]
    MiscError(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/// HcBundle Result type.
pub type HcLaunchResult<T> = Result<T, HcLaunchError>;