use std::fs;

use holochain_conductor_client::{AdminWebsocket, AppStatusFilter};
use tauri::api::process::{Command, CommandEvent};

use crate::{setup::config::caddyfile_path, uis::port_mapping::app_ui_folder_path};

use super::port_mapping::PortMapping;

const LAUNCHER_ENV_URL: &str = ".launcher-env.json";

fn caddyfile_config_for_an_app(
  admin_interface_port: u16,
  app_interface_port: u16,
  ui_port: u16,
  app_id: String,
) -> String {
  format!(
    r#":{} {{
        respond /{} 200 {{
                body `{{
                        "APP_INTERFACE_PORT": {},
                        "ADMIN_INTERFACE_PORT": {},
                        "INSTALLED_APP_ID": "{}"
                }}`
                close
        }}
        
        header Cache-Control no-cache, no-store
        
        root * "{}"
        file_server
}}
"#,
    ui_port,
    LAUNCHER_ENV_URL,
    app_interface_port,
    admin_interface_port,
    app_id.clone(),
    app_ui_folder_path(app_id)
      .into_os_string()
      .to_str()
      .unwrap(),
  )
}

fn build_caddyfile_contents(
  admin_port: u16,
  app_interface_port: u16,
  active_apps_ids: Vec<String>,
  port_mapping: PortMapping,
) -> Result<String, String> {
  let config_vec = active_apps_ids
    .into_iter()
    .map(|app_id| {
      let ui_port = port_mapping
        .get_ui_port_for_app(&app_id)
        .ok_or(String::from("This app has no assigned port"))?;

      Ok(caddyfile_config_for_an_app(
        admin_port,
        app_interface_port,
        ui_port,
        app_id,
      ))
    })
    .collect::<Result<Vec<String>, String>>()?;

  let empty_line = r#"
"#;

  if config_vec.len() == 0 {
    return Ok(String::from(empty_line));
  }

  Ok(config_vec.join(empty_line))
}

/// Connects to the conductor, requests the list of running apps, and writes the Caddyfile with the appropriate port mapping
async fn refresh_caddyfile(admin_port: u16) -> Result<(), String> {
  log::info!("Refreshing caddyfile");
  let mut ws = AdminWebsocket::connect(format!("ws://localhost:{}", admin_port))
    .await
    .or(Err(String::from("Could not connect to conductor")))?;

  let active_apps = ws
    .list_apps(Some(AppStatusFilter::Running))
    .await
    .or(Err("Could not get the currently active apps"))?;
  let app_interfaces = ws
    .list_app_interfaces()
    .await
    .or(Err(String::from("Could not list app interfaces")))?;

  let port_mapping = PortMapping::read_port_mapping()?;

  let active_app_ids = active_apps
    .into_iter()
    .map(|a| a.installed_app_id)
    .collect();

  let caddyfile_contents =
    build_caddyfile_contents(admin_port, app_interfaces[0], active_app_ids, port_mapping)?;

  fs::write(caddyfile_path(), caddyfile_contents)
    .map_err(|err| format!("Error writing Caddyfile: {:?}", err))?;

  Ok(())
}

/// Refreshes the running apps and reloads caddy to be consistent with them
/// Execute this when there has been some change in the status of an app (enabled, disabled, uninstalled...)
pub async fn reload_caddy(admin_port: u16) -> Result<(), String> {
  refresh_caddyfile(admin_port).await?;

  log::info!("Reloading Caddy");

  Command::new_sidecar("caddy")
    .or(Err(String::from("Can't find caddy binary")))?
    .args(&[
      "reload",
      "--config",
      caddyfile_path().as_os_str().to_str().unwrap(),
    ])
    .spawn()
    .map_err(|err| format!("Error reloading caddy {:?}", err))?;

  Ok(())
}

/// Builds the Caddyfile from the list of running apps and launches caddy
/// Execute only on launcher start
pub async fn launch_caddy(admin_port: u16) -> Result<(), String> {
  refresh_caddyfile(admin_port).await?;

  let (mut caddy_rx, _) = Command::new_sidecar("caddy")
    .or(Err(String::from("Can't find caddy binary")))?
    .args(&[
      "run",
      "--config",
      caddyfile_path().as_os_str().to_str().unwrap(),
    ])
    .spawn()
    .map_err(|err| format!("Error running caddy {:?}", err))?;

  tauri::async_runtime::spawn(async move {
    // read events such as stdout
    while let Some(event) = caddy_rx.recv().await {
      match event.clone() {
        CommandEvent::Stdout(line) => log::info!("[CADDY] {}", line),
        CommandEvent::Stderr(line) => log::info!("[CADDY] {}", line),
        _ => log::info!("[CADDY] {:?}", event),
      }
    }
  });
  log::info!("Launched caddy");

  Ok(())
}
