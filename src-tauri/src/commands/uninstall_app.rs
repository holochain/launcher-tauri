use crate::launcher::{state::LauncherState, manager::HolochainId};
use tauri::Manager;
use holochain_web_app_manager::derive_window_label;

#[tauri::command]
pub async fn uninstall_app(
  app_handle: tauri::AppHandle,
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  app_id: String,
  holochain_id: HolochainId,
) -> Result<(), String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command. (P)"))
  }

  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  manager
    .get_web_happ_manager(holochain_id.clone())?
    .uninstall_app(app_id.clone())
    .await?;

  manager.on_apps_changed().await?;

  // close existing window belonging to that app if there is one
  let window_label = derive_window_label(&app_id);
  if let Some(w) = app_handle.get_window(window_label.as_str()) {
    w.close().map_err(|e| format!("Failed to close app window after uninstalling app: {:?}", e))?;
  }

  Ok(())
}
