use std::{fs, io, path::PathBuf};

use tauri::api::process::kill_children;

use crate::{
  launch::launch_children_processes,
  setup::config::{holochain_config_path, holochain_data_path},
};

#[tauri::command]
pub async fn execute_factory_reset() -> Result<(), String> {
  factory_reset().await
}

async fn factory_reset() -> Result<(), String> {
  log::warn!("A factory reset has been requested, initiating...");

  // Kill all the children processes to avoid messing up with the filesystem
  kill_children();
  log::info!("Stopped children processes");

  remove_dir_if_exists(holochain_config_path()).map_err(|err| {
    log::error!("Could not remove holochain config path: {}", err);
    String::from("Could not remove holochain config path")
  })?;
  remove_dir_if_exists(holochain_data_path())
    .or(Err(String::from("Could not remove holochain data path")))?;

  launch_children_processes().await.map_err(|err| {
    log::error!("Failed to restart Holochain: {}", err);
    String::from("Failed to restart Holochain")
  })?;

  log::info!("Started children processes again, factory reset completed");

  Ok(())
}

fn remove_dir_if_exists(path: PathBuf) -> io::Result<()> {
  if let Ok(_) = fs::read_dir(path.clone()) {
    fs::remove_dir_all(path)?;
  }
  Ok(())
}
