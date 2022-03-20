use std::{collections::HashMap, path::PathBuf};
use tauri::api::process::{Command, CommandEvent};

pub fn launch_lair_keystore_process(
  log_level: log::Level,
  keystore_data_path: PathBuf,
) -> Result<(), String> {
  let mut envs = HashMap::new();
  envs.insert(String::from("RUST_LOG"), String::from(log_level.as_str()));

  let (mut lair_rx, _) = Command::new_sidecar("lair-keystore")
    .or(Err(String::from("Can't find lair-keystore binary")))?
    .args(&["-d", keystore_data_path.into_os_string().to_str().unwrap()])
    .envs(envs.clone())
    .spawn()
    .map_err(|err| format!("Failed to execute lair-keystore: {:?}", err))?;

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

pub fn launch_holochain_process(
  log_level: log::Level,
  conductor_config_path: PathBuf,
) -> Result<(), String> {
  let mut envs = HashMap::new();
  envs.insert(String::from("RUST_LOG"), String::from(log_level.as_str()));

  let (mut holochain_rx, holochain_child) = Command::new_sidecar("holochain")
    .or(Err(String::from("Can't find holochain binary")))?
    .args(&[
      "-c",
      conductor_config_path.into_os_string().to_str().unwrap(),
    ])
    .envs(envs)
    .spawn()
    .map_err(|err| format!("Failed to execute holochain: {:?}", err))?;

  tauri::async_runtime::spawn(async move {
    // read events such as stdout
    while let Some(event) = holochain_rx.recv().await {
      match event.clone() {
        CommandEvent::Stdout(line) => log::info!("[HOLOCHAIN] {}", line),
        CommandEvent::Stderr(line) => log::info!("[HOLOCHAIN] {}", line),
        _ => log::info!("[HOLOCHAIN] {:?}", event),
      };
      if format!("{:?}", event).contains("Installing lair_keystore") {
        // Lair keystore can't be executed, Holochain is trying to download and install Lair, kill it
        log::error!("Holochain is trying to download and install lair_keystore directly! Killing Holochain...");
        let result = holochain_child.kill();
        log::error!("Holochain terminated: {:?}", result);
        break;
      }
    }
  });
  log::info!("Launched holochain");

  Ok(())
}
