use crate::state::LauncherState;

#[tauri::command]
pub async fn get_connection_status(
  state: tauri::State<'_, LauncherState>,
) -> Result<String, String> {
  Ok(state.get_connection_status().await)
}
