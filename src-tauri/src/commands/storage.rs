use holochain_web_app_manager::StorageInfo;

use crate::launcher::{state::LauncherState, manager::HolochainId};

#[tauri::command]
pub async fn get_storage_info(
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  holochain_id: HolochainId,
) -> Result<StorageInfo, String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command. (H)"))
  }
  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  let storage_info = manager
    .get_web_happ_manager(holochain_id)?
    .get_storage_info()?;

  Ok(storage_info)
}
