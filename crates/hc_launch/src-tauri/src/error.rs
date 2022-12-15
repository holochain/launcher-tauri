use thiserror;
use serde::{Deserialize, Serialize};


#[derive(Debug, thiserror::Error, Serialize, Deserialize)]
pub enum HcLaunchError {
    #[error("Specified UI path \"{0}\" does not exist.")]
    UiPathDoesNotExist(String),

    #[error("Failed to get data to sign from unsigned zome call: \"{0}\"")]
    DataToSignError(String),

    #[error("Failed to sign by public key: \"{0}\"")]
    SignZomeCallError(String),
}

/// HcBundle Result type.
pub type HcLaunchResult<T> = Result<T, HcLaunchError>;