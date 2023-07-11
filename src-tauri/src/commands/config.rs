use holochain_manager::versions::common::{bootstrap_service, signaling_server};
use tauri::Manager;
use crate::launcher::{config::LauncherConfig, error::LauncherError};
use tauri::api::process;


#[tauri::command]
pub async fn write_config(
  window: tauri::Window,
  app_handle: tauri::AppHandle,
  config: LauncherConfig,
) -> Result<(), LauncherError> {

  if window.label() != "admin" {
    return Err(LauncherError::Unauthorized("Unauthorized: Attempted to call an tauri command 'write_config' which is not allowed in that window.".into()))
  }

  config.write()?;

  let windows = app_handle.windows();

  for (label, w) in windows {
    if !label.eq(&String::from("admin")) {
      if let Err(err) = w.close() {
        log::error!("Error closing window {:?}", err);
      }
    }
  }

  process::kill_children();
  app_handle.restart();

  Ok(())
}




#[tauri::command]
pub async fn get_default_bootstrap(
  window: tauri::Window,
) -> Result<String, LauncherError> {

  if window.label() != "admin" {
    return Err(LauncherError::Unauthorized("Unauthorized: Attempted to call an tauri command 'get_default_bootstrap' which is not allowed in that window.".into()))
  }

  Ok(bootstrap_service().to_string())
}


#[tauri::command]
pub async fn get_default_signaling(
  window: tauri::Window,
) -> Result<String, LauncherError> {

  if window.label() != "admin" {
    return Err(LauncherError::Unauthorized("Unauthorized: Attempted to call an tauri command 'get_default_signaling' which is not allowed in that window.".into()))
  }

  Ok(signaling_server())
}