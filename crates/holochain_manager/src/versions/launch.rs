use log;
use std::{collections::HashMap, path::PathBuf};
use tauri::api::process::{Command, CommandEvent, CommandChild};

use lair_keystore_manager::error::LaunchChildError;

use super::HolochainVersion;
use crate::error::LaunchHolochainError;

pub fn launch_holochain_process(
  log_level: log::Level,
  holochain_version: HolochainVersion,
  conductor_config_path: PathBuf,
  password: String,
) -> Result<CommandChild, LaunchHolochainError> {
  let mut envs = HashMap::new();
  envs.insert(String::from("RUST_LOG"), String::from(log_level.as_str()));
  envs.insert(String::from("WASM_LOG"), String::from(log_level.as_str()));

  let (mut holochain_rx, mut holochain_child) =
    Command::new_sidecar("holochain") // TODO: Fix
      .or(Err(LaunchHolochainError::LaunchChildError(
        LaunchChildError::BinaryNotFound,
      )))?
      .args(&[
        "-c",
        conductor_config_path.into_os_string().to_str().unwrap(),
        "-p",
      ])
      .envs(envs)
      .spawn()
      .map_err(|err| {
        LaunchHolochainError::LaunchChildError(LaunchChildError::FailedToExecute(
          format!("{}", err),
        ))
      })?;

  tauri::async_runtime::spawn(async move {
    // read events such as stdout
    while let Some(event) = holochain_rx.recv().await {
      match event.clone() {
        CommandEvent::Stdout(line) => log::info!("[HOLOCHAIN] {}", line),
        CommandEvent::Stderr(line) => log::info!("[HOLOCHAIN] {}", line),
        _ => log::info!("[HOLOCHAIN] {:?}", event),
      };
    }
  });
  log::info!("Launched holochain");

  holochain_child
    .write(password.as_bytes())
    .map_err(|err| LaunchHolochainError::ErrorWritingPassword(format!("{:?}", err)))?;
  holochain_child
    .write("\n".as_bytes())
    .map_err(|err| LaunchHolochainError::ErrorWritingPassword(format!("{:?}", err)))?;

  Ok(holochain_child)
}
