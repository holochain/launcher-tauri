use crate::uis::{caddy};

#[tauri::command]
pub async fn activate_app_ui(app_id: String) -> Result<(), String> {
  log::info!("Activating UI: app_id = {}", app_id);

  caddy::reload_caddy().await
}

#[tauri::command]
pub async fn deactivate_app_ui(app_id: String) -> Result<(), String> {
  log::info!("Deactivating UI: app_id = {}", app_id);

  caddy::reload_caddy().await
}
