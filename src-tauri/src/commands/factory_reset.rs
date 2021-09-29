use std::{fs, io, path::PathBuf};

use tauri::api::process::kill_children;

use crate::{
  launch::launch_children_processes,
  setup::config::{holochain_config_path, holochain_data_path, keystore_data_path},
  state::LauncherState,
};

#[tauri::command]
pub async fn execute_factory_reset(state: tauri::State<'_, LauncherState>) -> Result<(), String> {
  let admin_port = state.connection_status.get_admin_port()?;

  factory_reset(admin_port).await
}

async fn factory_reset(admin_port: u16) -> Result<(), String> {
  log::warn!("A factory reset has been requested, initiating...");

  // Kill all the children processes to avoid messing up with the filesystem
  kill_children();
  log::info!("Stopped children processes");

  remove_dir_if_exists(holochain_config_path()).map_err(|err| {
    log::error!("Could not remove holochain config path: {}", err);
    String::from("Could not remove holochain config path")
  })?;
  remove_dir_if_exists(keystore_data_path()).map_err(|err| {
    log::error!("Could not remove lair path: {}", err);
    String::from("Could not remove lair path")
  })?;
  remove_dir_if_exists(holochain_data_path())
    .or(Err(String::from("Could not remove holochain data path")))?;

  launch_children_processes(admin_port).await.map_err(|err| {
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
