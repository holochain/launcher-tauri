use std::{fs, io, path::PathBuf};

use tauri::{api::process::kill_children, Manager};

use crate::{
  file_system::{profile_config_dir, profile_holochain_data_dir, profile_lair_dir, Profile, profile_logs_dir, profile_tauri_dir},
  launcher::{error::LauncherError, manager::LauncherManager, state::LauncherState},
  running_state::RunningState,
};

#[tauri::command]
pub async fn execute_factory_reset(
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  profile: tauri::State<'_, Profile>,
  app_handle: tauri::AppHandle,
  delete_logs: bool,
) -> Result<(), String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command. (E)"))
  }

  log::warn!("A factory reset has been requested, initiating...");
  // clear localStorage (TODO! ideally this would be at a better location on the filesystem and could be deleted
  // by deleting the corresponding directory)
  if let Err(e) = window.eval("window.localStorage.clear()") {
    log::error!("Error clearing localStorage: {:?}", e);
  }


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

  let profile = profile.inner().clone();

  let config_dir = profile_config_dir(profile.clone())
    .map_err(|e| format!("Failed to get config dir: {}", e))?;

  remove_dir_if_exists(config_dir).map_err(|err| {
    log::error!("Could not remove holochain config directory: {}", err);
    format!("Could not remove holochain config directory: {}", err)
  })?;


  let holochain_data_dir = profile_holochain_data_dir(profile.clone())
    .map_err(|e| format!("Failed to get holochain data dir: {}", e))?;

  remove_dir_if_exists(holochain_data_dir)
    .map_err(|err| {
      log::error!("Could not remove holochain data directory: {}", err);
      format!("Could not remove holochain data directory: {}", err)
  })?;


  let tauri_data_dir = profile_tauri_dir(profile.clone())
    .map_err(|e| format!("Failed to get tauri data dir: {}", e))?;

  remove_dir_if_exists(tauri_data_dir)
    .map_err(|err| {
      log::error!("Could not remove tauri data directory: {}", err);
      format!("Could not remove tauri data directory: {}", err)
  })?;


  let lair_dir = profile_lair_dir(profile.clone())
    .map_err(|e| format!("Failed to get lair dir: {}", e))?;

  remove_dir_if_exists(lair_dir).map_err(|err| {
    log::error!("Could not remove lair directory: {}", err);
    String::from("Could not remove lair directory")
  })?;


  // Optional deletions

  if delete_logs == true {
    let logs_dir = profile_logs_dir(profile.clone())
      .map_err(|e| format!("Failed to get logs dir: {}", e))?;

    remove_dir_if_exists(logs_dir).map_err(|err| {
      log::error!("Could not remove logs directory: {}", err);
      String::from("Could not remove logs directory")
    })?;
  }



  let manager_launch = LauncherManager::launch(app_handle, profile.clone()).await;

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
