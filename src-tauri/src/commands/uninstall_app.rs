use crate::launcher::{state::LauncherState, manager::HolochainId};

#[tauri::command]
pub async fn uninstall_app(
  state: tauri::State<'_, LauncherState>,
  app_id: String,
  holochain_id: HolochainId,
) -> Result<(), String> {
  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  manager
    .get_web_happ_manager(holochain_id)?
    .uninstall_app(app_id)
    .await?;

  manager.on_apps_changed().await?;

  Ok(())
}
