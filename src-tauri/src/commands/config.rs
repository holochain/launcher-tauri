use tauri::{api::process::kill_children, Manager};

use crate::{
  launcher::{
    config::LauncherConfig, error::LauncherError, manager::LauncherManager, state::LauncherState,
  },
  running_state::RunningState,
};

#[tauri::command]
pub async fn write_config(
  state: tauri::State<'_, LauncherState>,
  app_handle: tauri::AppHandle,
  config: LauncherConfig,
) -> Result<(), LauncherError> {
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

  let manager = LauncherManager::launch(app_handle).await?;

  let mut m = state.lock().await;

  (*m) = RunningState::Running(manager);

  Ok(())
}
