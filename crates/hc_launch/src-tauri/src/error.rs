use thiserror;
use serde::{Deserialize, Serialize};
use std::{fmt, io};


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



#[derive(thiserror::Error, Debug, Serialize, Deserialize, Clone)]
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