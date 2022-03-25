use std::collections::HashMap;
use std::path::PathBuf;

use tauri::api::process::{Command, CommandEvent};

use crate::error::LaunchTauriSidecarError;

pub fn launch_lair_keystore_process(
    log_level: log::Level,
    keystore_data_path: PathBuf,
) -> Result<(), LaunchTauriSidecarError> {
    let mut envs = HashMap::new();
    envs.insert(String::from("RUST_LOG"), String::from(log_level.as_str()));

    let (mut lair_rx, _) = Command::new_sidecar("lair-keystore")
        .or(Err(LaunchTauriSidecarError::BinaryNotFound))?
        .args(&["-d", keystore_data_path.into_os_string().to_str().unwrap()])
        .envs(envs.clone())
        .spawn()
        .map_err(|err| LaunchTauriSidecarError::FailedToExecute(format!("{:?}", err)))?;

    tauri::async_runtime::spawn(async move {
        // read events such as stdout
        while let Some(event) = lair_rx.recv().await {
            match event.clone() {
                CommandEvent::Stdout(line) => log::info!("[LAIR] {}", line),
                CommandEvent::Stderr(line) => log::info!("[LAIR] {}", line),
                _ => log::info!("[LAIR] {:?}", event),
            }
        }
    });

    log::info!("Launched lair-keystore");

    Ok(())
}
