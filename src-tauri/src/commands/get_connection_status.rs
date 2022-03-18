use crate::state::LauncherState;

#[tauri::command]
pub fn get_connection_status(state: tauri::State<'_, LauncherState>) -> String {
  (*state).get_connection_status()
}
