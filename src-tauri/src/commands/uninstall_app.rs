use crate::state::LauncherState;

#[tauri::command]
pub async fn uninstall_app(
  state: tauri::State<'_, LauncherState>,
  app_id: String,
  app_handle: tauri::AppHandle,
) -> Result<(), String> {
  let manager = state.get_holochain_manager()?;

  manager.uninstall_app(app_id);

  state.get_launcher_manager()?.on_apps_changed(&app_handle);

  Ok(())
}
