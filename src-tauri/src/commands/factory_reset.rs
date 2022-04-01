use std::{fs, io, path::PathBuf};

use tauri::api::process::kill_children;

use crate::{
  file_system::{root_config_path, root_data_path, root_lair_path},
  launcher::{manager::LauncherManager, state::LauncherState},
  running_state::RunningState,
};

#[tauri::command]
pub async fn execute_factory_reset(
  state: tauri::State<'_, LauncherState>,
  app_handle: tauri::AppHandle,
) -> Result<(), String> {
  log::warn!("A factory reset has been requested, initiating...");

  // Kill all the children processes to avoid messing up with the filesystem
  kill_children();
  log::info!("Stopped children processes");

  remove_dir_if_exists(root_config_path()).map_err(|err| {
    log::error!("Could not remove holochain config path: {}", err);
    String::from("Could not remove holochain config path")
  })?;
  remove_dir_if_exists(root_lair_path()).map_err(|err| {
    log::error!("Could not remove lair path: {}", err);
    String::from("Could not remove lair path")
  })?;
  remove_dir_if_exists(root_data_path())
    .or(Err(String::from("Could not remove holochain data path")))?;

  let manager = LauncherManager::launch(app_handle).await?;

  let mut m = state.lock().await;

  (*m) = RunningState::Running(manager);

  log::info!("Started children processes again, factory reset completed");

  Ok(())
}

fn remove_dir_if_exists(path: PathBuf) -> io::Result<()> {
  if let Ok(_) = fs::read_dir(path.clone()) {
    fs::remove_dir_all(path)?;
  }
  Ok(())
}
