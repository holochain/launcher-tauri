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
