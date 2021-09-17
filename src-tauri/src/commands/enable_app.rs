use holochain_conductor_client::AdminWebsocket;

use crate::{config::admin_url, uis::caddy};

#[tauri::command]
pub async fn enable_app(app_id: String) -> Result<(), String> {
  let mut ws = AdminWebsocket::connect(admin_url())
    .await
    .or(Err(String::from("Could not connect to conductor")))?;

  ws.enable_app(app_id.clone())
    .await
    .map_err(|err| format!("Error enabling app: {:?}", err))?;

  log::info!("Activating UI: app_id = {}", app_id);

  caddy::reload_caddy().await
}

#[tauri::command]
pub async fn disable_app(app_id: String) -> Result<(), String> {
  let mut ws = AdminWebsocket::connect(admin_url())
    .await
    .or(Err(String::from("Could not connect to conductor")))?;

  ws.disable_app(app_id.clone())
    .await
    .map_err(|err| format!("Error disabling app: {:?}", err))?;

  log::info!("Deactivating UI: app_id = {}", app_id);

  caddy::reload_caddy().await
}
