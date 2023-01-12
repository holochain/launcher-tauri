use std::path::PathBuf;
use std::{collections::HashMap, time::Duration};
use url2::Url2;

use tauri::api::process::{Command, CommandEvent};

use crate::error::{LairKeystoreError, LaunchChildError};

pub async fn launch_lair_keystore_process(
  log_level: log::Level,
  keystore_data_dir: PathBuf,
  password: String,
) -> Result<Url2, LairKeystoreError> {
  let mut envs = HashMap::new();
  envs.insert(String::from("RUST_LOG"), String::from(log_level.as_str()));

  // NEW_VERSION Check whether lair-keystore version needs to get updated
  let (mut lair_rx, mut command_child) = Command::new_sidecar("lair-keystore-v0.2.3")
    .or(Err(LairKeystoreError::LaunchChildError(
      LaunchChildError::BinaryNotFound,
    )))?
    .args(&["server", "-p"])
    .current_dir(keystore_data_dir.clone())
    .envs(envs.clone())
    .spawn()
    .map_err(|err| {
      LairKeystoreError::LaunchChildError(LaunchChildError::FailedToExecute(format!("{:?}", err)))
    })?;

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
          if line.contains("lair-keystore running") {
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

  tauri::async_runtime::spawn(async move {
    // read events such as stdout
    while let Some(event) = lair_rx.recv().await {
      match event.clone() {
        CommandEvent::Stdout(line) => log::info!("[LAIR] {}", line),
        CommandEvent::Stderr(line) => log::error!("[LAIR] {}", line),
        _ => log::info!("[LAIR] {:?}", event),
      }
    }
  });

  // NEW_VERSION Check whether lair-keystore version needs to get updated
  let output = Command::new_sidecar("lair-keystore-v0.2.3")
    .or(Err(LairKeystoreError::LaunchChildError(
      LaunchChildError::BinaryNotFound,
    )))?
    .args(&["url"])
    .current_dir(keystore_data_dir)
    .envs(envs.clone())
    .output()
    .map_err(|err| {
      LairKeystoreError::LaunchChildError(LaunchChildError::FailedToExecute(format!("{:?}", err)))
    })?;

  if output.stderr.len() > 0 {
    return Err(LairKeystoreError::LaunchChildError(
      LaunchChildError::FailedToExecute(output.stderr),
    ));
  }

  let url = Url2::parse(output.stdout);

  log::info!("Launched lair-keystore");

  Ok(url)
}
