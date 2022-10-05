use std::{fs, io, path::PathBuf};

use tauri::{api::process::kill_children, Manager};

use crate::{
  file_system::{root_config_path, root_holochain_data_path, root_lair_path},
  launcher::{error::LauncherError, manager::LauncherManager, state::LauncherState},
  running_state::RunningState,
};

#[tauri::command]
pub async fn execute_factory_reset(
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  app_handle: tauri::AppHandle,
) -> Result<(), String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command. (E)"))
  }

  log::warn!("A factory reset has been requested, initiating...");

  let windows = app_handle.windows();

  for (label, w) in windows {
    if !label.eq(&String::from("admin")) {
      if let Err(err) = w.close() {
        log::error!("Error closing window {:?}", err);
      }
    }
  }

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
  remove_dir_if_exists(root_holochain_data_path())
    .or(Err(String::from("Could not remove holochain data path")))?;

  let manager_launch = LauncherManager::launch(app_handle).await;

  let mut maybe_error: Option<LauncherError> = None;

  let manager_state = match manager_launch {
    Ok(mut launcher_manager) => {
      log::info!("Launch setup successful");
      launcher_manager.on_apps_changed().await?;

      RunningState::Running(launcher_manager)
    }
    Err(error) => {
      kill_children();

      maybe_error = Some(error.clone());

      log::error!("There was an error launching holochain: {:?}", error);
      RunningState::Error(error)
    }
  };

  let mut m = state.lock().await;

  (*m) = manager_state;

  log::info!("Started children processes again, factory reset completed");

  if let Some(err) = maybe_error {
    return Err(format!("{:?}", err));
  }

  Ok(())
}

fn remove_dir_if_exists(path: PathBuf) -> io::Result<()> {
  if let Ok(_) = fs::read_dir(path.clone()) {
    fs::remove_dir_all(path)?;
  }
  Ok(())
}
