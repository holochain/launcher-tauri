use holochain_manager::versions::HolochainVersion;

use crate::launcher::state::LauncherState;

#[tauri::command]
pub async fn start_app(
  state: tauri::State<'_, LauncherState>,
  app_id: String,
  holochain_version: HolochainVersion,
) -> Result<(), String> {
  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  manager
    .get_web_happ_manager(holochain_version)?
    .start_app(app_id)
    .await?;

  manager.on_apps_changed().await?;

  Ok(())
}
