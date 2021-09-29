use holochain_conductor_client::AdminWebsocket;

use crate::{state::LauncherState, uis::caddy};

#[tauri::command]
pub async fn enable_app(
  state: tauri::State<'_, LauncherState>,
  app_id: String,
) -> Result<(), String> {
  let admin_port = state.connection_status.get_admin_port()?;

  let mut ws = AdminWebsocket::connect(format!("ws://localhost:{}", admin_port))
    .await
    .or(Err(String::from("Could not connect to conductor")))?;

  ws.enable_app(app_id.clone())
    .await
    .map_err(|err| format!("Error enabling app: {:?}", err))?;

  log::info!("Activating UI: app_id = {}", app_id);

  caddy::reload_caddy(admin_port).await
}

#[tauri::command]
pub async fn disable_app(
  state: tauri::State<'_, LauncherState>,
  app_id: String,
) -> Result<(), String> {
  let admin_port = state.connection_status.get_admin_port()?;

  let mut ws = AdminWebsocket::connect(format!("ws://localhost:{}", admin_port))
    .await
    .or(Err(String::from("Could not connect to conductor")))?;

  ws.disable_app(app_id.clone())
    .await
    .map_err(|err| format!("Error disabling app: {:?}", err))?;

  log::info!("Deactivating UI: app_id = {}", app_id);

  caddy::reload_caddy(admin_port).await
}
