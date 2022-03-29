use std::collections::HashMap;
use std::path::PathBuf;
use url2::Url2;

use tauri::api::process::{Command, CommandEvent};

use crate::error::{LairKeystoreError, LaunchTauriSidecarError};

pub fn launch_lair_keystore_process(
  log_level: log::Level,
  keystore_data_path: PathBuf,
  password: String,
) -> Result<Url2, LairKeystoreError> {
  let mut envs = HashMap::new();
  envs.insert(String::from("RUST_LOG"), String::from(log_level.as_str()));

  let (mut lair_rx, mut command_child) = Command::new_sidecar("lair-keystore")
    .or(Err(LairKeystoreError::LaunchTauriSidecarError(
      LaunchTauriSidecarError::BinaryNotFound,
    )))?
    .args(&["server", "-p"])
    .current_dir(keystore_data_path.clone())
    .envs(envs.clone())
    .spawn()
    .map_err(|err| {
      LairKeystoreError::LaunchTauriSidecarError(LaunchTauriSidecarError::FailedToExecute(format!(
        "{:?}",
        err
      )))
    })?;

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

  command_child
    .write(password.as_bytes())
    .map_err(|err| LairKeystoreError::ErrorWritingPassword(format!("{:?}", err)))?;
  command_child
    .write(&[ascii::AsciiChar::EOT.as_byte()])
    .map_err(|err| LairKeystoreError::ErrorWritingPassword(format!("{:?}", err)))?;

  let output = Command::new_sidecar("lair-keystore")
    .or(Err(LairKeystoreError::LaunchTauriSidecarError(
      LaunchTauriSidecarError::BinaryNotFound,
    )))?
    .args(&["url"])
    .current_dir(keystore_data_path)
    .envs(envs.clone())
    .output()
    .map_err(|err| {
      LairKeystoreError::LaunchTauriSidecarError(LaunchTauriSidecarError::FailedToExecute(format!(
        "{:?}",
        err
      )))
    })?;

  if output.stderr.len() > 0 {
    return Err(LairKeystoreError::LaunchTauriSidecarError(
      LaunchTauriSidecarError::FailedToExecute(output.stderr),
    ));
  }

  let url = Url2::parse(output.stdout);

  log::info!("Launched lair-keystore");

  Ok(url)
}
