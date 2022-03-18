use holochain_client::AdminWebsocket;

use crate::state::LauncherState;

#[tauri::command]
pub async fn enable_app(
  state: tauri::State<'_, LauncherState>,
  app_id: String,
  app_handle: tauri::AppHandle,
) -> Result<(), String> {
  let manager = (*state).get_holochain_manager()?;

  let ports = state.get_running_ports()?;

  let mut ws = AdminWebsocket::connect(format!("ws://localhost:{}", ports.admin_interface_port))
    .await
    .or(Err(String::from("Could not connect to conductor")))?;

  ws.enable_app(app_id.clone())
    .await
    .map_err(|err| format!("Error enabling app: {:?}", err))?;

  log::info!("Activating UI: app_id = {}", app_id);

  caddy::reload_caddy(ports).await
}

#[tauri::command]
pub async fn disable_app(
  state: tauri::State<'_, LauncherState>,
  app_id: String,
) -> Result<(), String> {
  let ports = state.get_running_ports()?;

  let mut ws = AdminWebsocket::connect(format!("ws://localhost:{}", ports.admin_interface_port))
    .await
    .or(Err(String::from("Could not connect to conductor")))?;

  ws.disable_app(app_id.clone())
    .await
    .map_err(|err| format!("Error disabling app: {:?}", err))?;

  log::info!("Deactivating UI: app_id = {}", app_id);

  caddy::reload_caddy(ports).await
}
