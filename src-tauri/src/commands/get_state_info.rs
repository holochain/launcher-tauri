use crate::launcher::state::{LauncherState, LauncherStateInfo};

#[tauri::command]
pub async fn get_state_info(
  state: tauri::State<'_, LauncherState>,
) -> Result<LauncherStateInfo, String> {
  state.get_state_info().await
}
