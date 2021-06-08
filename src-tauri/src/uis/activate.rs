use super::port_mapping::{app_ui_folder_path, PortMapping};
use holochain_conductor_api_rust::AdminWebsocket;
use std::process::Command;

pub fn activate_app_ui(app_id: String, port: u16) -> Result<(), String> {
  Command::new("caddy")
    .arg("file-server")
    .args(&[
      "--root",
      app_ui_folder_path(app_id).as_os_str().to_str().unwrap(),
    ])
    .args(&["--listen", format!("localhost:{}", port).as_str()])
    .spawn()
    .expect("failed to execute process");

  Ok(())
}

pub async fn activate_uis_for_active_apps(ws: &mut AdminWebsocket) -> Result<(), String> {
  let active_app_ids = ws
    .list_active_apps()
    .await
    .or(Err("Could not get the currently active apps"))?;

  let port_mapping = PortMapping::read_port_mapping()?;

  for app_id in active_app_ids {
    let port = port_mapping
      .get_ui_port_for_app(&app_id)
      .ok_or(String::from("Couldn't find active app in port mappings"))?;
    activate_app_ui(app_id, port)?;
  }

  Ok(())
}
