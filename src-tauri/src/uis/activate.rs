use super::port_mapping::{app_ui_folder_path, PortMapping};
use holochain_conductor_api_rust::AdminWebsocket;
/* use std::sync::Arc;
 */
use tauri::api::process::Command;

#[tauri::command]
pub fn activate_app_ui(app_id: String) -> Result<(), String> {
  let port_mapping = PortMapping::read_port_mapping()?;

  inner_activate_app_ui(&port_mapping, app_id)
}

pub fn inner_activate_app_ui(port_mapping: &PortMapping, app_id: String) -> Result<(), String> {
  let port = port_mapping
    .get_ui_port_for_app(&app_id)
    .ok_or(String::from("Couldn't find active app in port mappings"))?;

  Command::new_sidecar("caddy")
    .or(Err(String::from("Can't find caddy binary")))?
    .args(&[
      "file-server",
      "--root",
      app_ui_folder_path(app_id.clone())
        .as_os_str()
        .to_str()
        .unwrap(),
      "--listen",
      format!("localhost:{}", port).as_str(),
    ])
    .spawn()
    .map_err(|err| format!("Failed to execute caddy file-server: {:?}", err))?;

  /*  let arc = Arc::clone(&state.caddy_processes);
  let mut caddy_processes = arc.lock().unwrap();

  caddy_processes.insert(app_id, child); */

  Ok(())
}

pub async fn activate_uis_for_active_apps(ws: &mut AdminWebsocket) -> Result<(), String> {
  let active_app_ids = ws
    .list_active_apps()
    .await
    .or(Err("Could not get the currently active apps"))?;

  let port_mapping = PortMapping::read_port_mapping()?;

  for app_id in active_app_ids {
    inner_activate_app_ui(&port_mapping, app_id)?;
  }

  Ok(())
}
