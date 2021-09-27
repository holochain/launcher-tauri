use crate::state::LauncherState;

#[tauri::command]
pub async fn get_admin_port(state: tauri::State<'_, LauncherState>) -> Result<u16, String> {
  Ok(state.admin_interface_port)
}
