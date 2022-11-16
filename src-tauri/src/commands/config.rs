use tauri::{api::process::kill_children, Manager};

use crate::{
  launcher::{
    config::LauncherConfig, error::LauncherError, manager::LauncherManager, state::LauncherState,
  },
  running_state::RunningState,
};

#[tauri::command]
pub async fn write_config(
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  app_handle: tauri::AppHandle,
  config: LauncherConfig,
  custom_path: Option<String>,
) -> Result<(), LauncherError> {

  if window.label() != "admin" {
    return Err(LauncherError::Unauthorized("Unauthorized: Attempted to call an unauthorized tauri command. (C)".into()))
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

  kill_children();

  let manager = LauncherManager::launch(app_handle, custom_path).await?;

  let mut m = state.lock().await;

  (*m) = RunningState::Running(manager);

  Ok(())
}
