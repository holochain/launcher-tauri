use std::{fs, io, path::PathBuf};

use tauri::{api::process::kill_children, Manager};

use crate::file_system::{profile_config_dir, profile_holochain_data_dir, profile_lair_dir, Profile, profile_logs_dir, profile_tauri_dir, holochain_version_data_dir};

use holochain_manager::versions::HolochainVersion;
use strum::IntoEnumIterator;

#[tauri::command]
pub async fn execute_factory_reset(
  window: tauri::Window,
  profile: tauri::State<'_, Profile>,
  app_handle: tauri::AppHandle,
  delete_logs: bool,
  delete_all_holochain_versions: bool,
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

  // remove config files
  let config_dir = profile_config_dir(profile.clone())
    .map_err(|e| format!("Failed to get config dir: {}", e))?;

  remove_dir_if_exists(config_dir).map_err(|err| {
    log::error!("Could not remove holochain config directory: {}", err);
    format!("Could not remove holochain config directory: {}", err)
  })?;


  // remove holochain data
  if delete_all_holochain_versions == true {
    let holochain_data_dir = profile_holochain_data_dir(profile.clone())
      .map_err(|e| format!("Failed to get holochain data dir: {}", e))?;

    remove_dir_if_exists(holochain_data_dir)
      .map_err(|err| {
        log::error!("Could not remove holochain data directory: {}", err);
        format!("Could not remove holochain data directory: {}", err)
    })?;


    // !!! IMPORTANT !!! Lair is shared across holochain versions. ONLY DELETE IT if apps of all holochain versions are
    // supposed to be deleted
    let lair_dir = profile_lair_dir(profile.clone())
    .map_err(|e| format!("Failed to get lair dir: {}", e))?;

    remove_dir_if_exists(lair_dir).map_err(|err| {
      log::error!("Could not remove lair directory: {}", err);
      String::from("Could not remove lair directory")
    })?;

  } else {

    for version in HolochainVersion::iter() {
      let holochain_version_data_dir = holochain_version_data_dir(&version, profile.clone())
        .map_err(|e| {
          log::error!("Failed to get data directory of holochain version {:?} during factory reset: {:?}", version, e);
          format!("Failed to get data directory of holochain version {:?} during factory reset: {:?}", version, e)
        })?;

      remove_dir_if_exists(holochain_version_data_dir)
        .map_err(|err| {
          log::error!("Could not remove data directory of holochain version {:?}: {:?}", version, err);
          format!("Could not remove data directory of holochain version {:?}: {:?}", version, err)
      })?;
    }
  }


  if cfg!(not(target_os="windows")) {
    let tauri_data_dir = profile_tauri_dir(profile.clone())
      .map_err(|e| format!("Failed to get tauri data dir: {}", e))?;

    remove_dir_if_exists(tauri_data_dir)
      .map_err(|err| {
        log::error!("Could not remove tauri data directory: {}", err);
        format!("Could not remove tauri data directory: {}", err)
    })?;
  } else {
    // On Windows, deleting the tauri directory throws an os error 32, so instead just clear localStorage in the window
    window.eval("window.localStorage.clear()")
      .map_err(|e| format!("Failed to clear localStorage in admin window: {}", e))?;
  }


  // Optional deletions

  if delete_logs == true {
    let logs_dir = profile_logs_dir(profile.clone())
      .map_err(|e| format!("Failed to get logs dir: {}", e))?;

    remove_dir_if_exists(logs_dir).map_err(|err| {
      log::error!("Could not remove logs directory: {}", err);
      String::from("Could not remove logs directory")
    })?;
  }

  app_handle.restart();

  log::info!("Restarted Launcher, factory reset completed");

  Ok(())
}

fn remove_dir_if_exists(path: PathBuf) -> io::Result<()> {
  if let Ok(_) = fs::read_dir(path.clone()) {
    fs::remove_dir_all(path)?;
  }
  Ok(())
}
