use crate::state::{ConnectionStatus, LauncherState};

#[tauri::command]
pub fn get_connection_status(state: tauri::State<'_, LauncherState>) -> ConnectionStatus {
  (*state).connection_status.lock().unwrap().clone()
}
