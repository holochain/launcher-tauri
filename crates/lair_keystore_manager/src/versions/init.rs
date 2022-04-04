use std::path::{Path, PathBuf};

use tauri::api::process::Command;

use crate::error::{LairKeystoreError, LaunchChildError};

pub fn is_initialized(keystore_path: PathBuf) -> bool {
  Path::new(&keystore_path)
    .join("lair-keystore-config.yaml")
    .exists()
}

pub fn initialize(keystore_path: PathBuf, password: String) -> Result<(), LairKeystoreError> {
  let (mut _lair_rx, mut command_child) = Command::new_sidecar("lair-keystore")
    .or(Err(LairKeystoreError::LaunchChildError(
      LaunchChildError::BinaryNotFound,
    )))?
    .args(&["init", "-p"])
    .current_dir(keystore_path)
    .spawn()
    .map_err(|err| LaunchChildError::FailedToExecute(format!("{:?}", err)))?;

  command_child
    .write(password.as_bytes())
    .map_err(|err| LairKeystoreError::ErrorWritingPassword(format!("{:?}", err)))?;

  Ok(())
}
