use crate::launcher::{state::LauncherState, manager::HolochainId};


/// Stores the icon src of an app to the filesystem
#[tauri::command]
pub async fn store_icon_src(
  state: tauri::State<'_, LauncherState>,
  window: tauri::window::Window,
  app_id: String,
  holochain_id: HolochainId,
  icon_src: String,
) -> Result<(), String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call tauri command 'store_icon_src' which is not allowed in this window."))
  }

  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  let webapp_manager = manager.get_web_happ_manager(holochain_id)?;

  webapp_manager.store_app_icon_src(icon_src, &app_id)

}

/// Gets the icon src of an app from the filesystem
#[tauri::command]
pub async fn get_icon_src(
  state: tauri::State<'_, LauncherState>,
  window: tauri::window::Window,
  app_id: String,
  holochain_id: HolochainId,
) -> Result<Option<String>, String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call tauri command 'get_icon_src' which is not allowed in this window."))
  }

  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  let webapp_manager = manager.get_web_happ_manager(holochain_id)?;

  Ok(webapp_manager.get_app_icon_src(&app_id))

}