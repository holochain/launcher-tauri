use tauri::api::process::kill_children;

use crate::{
  launcher::{config::LauncherConfig, manager::LauncherManager, state::LauncherState},
  running_state::RunningState,
};

#[tauri::command]
pub async fn write_config(
  state: tauri::State<'_, LauncherState>,
  app_handle: tauri::AppHandle,
  config: LauncherConfig,
) -> Result<(), String> {
  config.write()?;

  kill_children();

  let manager = LauncherManager::launch(app_handle).await?;

  let mut m = state.lock().await;

  (*m) = RunningState::Running(manager);

  Ok(())
}
