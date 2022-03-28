use holochain_manager::{app_manager::AppManager, versions::HolochainVersion};

use crate::launcher::state::LauncherState;

#[tauri::command]
pub async fn uninstall_app(
  state: tauri::State<'_, LauncherState>,
  app_id: String,
  holochain_version: HolochainVersion,
  app_handle: tauri::AppHandle,
) -> Result<(), String> {
  let mut manager = state.get_launcher_manager()?.lock().await;

  manager
    .get_web_happ_manager(holochain_version)?
    .uninstall_app(app_id)
    .await?;

  manager.on_apps_changed().await?;

  Ok(())
}
