use std::{path::{Path, PathBuf}, time::Duration};

use tauri::api::process::{Command, CommandEvent};

use crate::error::{LairKeystoreError, LaunchChildError};

pub fn is_initialized(keystore_path: PathBuf) -> bool {
  Path::new(&keystore_path)
    .join("lair-keystore-config.yaml")
    .exists()
}

pub async fn initialize(keystore_path: PathBuf, password: String) -> Result<(), LairKeystoreError> {
  // NEW_VERSION Check whether lair-keystore version needs to get updated
  let (mut lair_rx, mut command_child) = Command::new_sidecar("lair-keystore-v0.3.0")
    .or(Err(LairKeystoreError::LaunchChildError(
      LaunchChildError::BinaryNotFound,
    )))?
    .args(&["init", "-p"])
    .current_dir(keystore_path)
    .spawn()
    .map_err(|err| LaunchChildError::FailedToExecute(format!("{:?}", err)))?;

  tauri::async_runtime::spawn(async move {
    std::thread::sleep(Duration::from_millis(10));
    command_child
      .write(password.as_bytes())
      .expect("Could not write password");
  });

  let mut started = false;
  while !started {
    if let Some(event) = lair_rx.recv().await {
      match event.clone() {
        CommandEvent::Stdout(line) => {
          log::info!("[LAIR] {}", line);
          if line.contains("lair-keystore init connection_url") {
            started = true;
          }
        }
        CommandEvent::Stderr(line) => {
          log::error!("[LAIR] {}", line);
          if line.contains("InternalSodium") {
            return Err(LairKeystoreError::IncorrectPassword);
          }
        }
        _ => {
          log::info!("[LAIR] {:?}", event);
        }
      }
    }
  }

  Ok(())
}
