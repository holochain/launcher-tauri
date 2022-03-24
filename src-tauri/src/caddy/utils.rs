use std::path::PathBuf;
use tauri::api::process::{Command, CommandEvent};

use super::manager::RunningWebApps;

pub fn launch_caddy_process(caddyfile_path: PathBuf) -> Result<(), String> {
  let (mut caddy_rx, _) = Command::new_sidecar("caddy")
    .or(Err(String::from("Can't find caddy binary")))?
    .args(&[
      "run",
      "--config",
      caddyfile_path.as_os_str().to_str().unwrap(),
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

pub fn reload_caddy(caddyfile_path: PathBuf) -> Result<(), String> {
  Command::new_sidecar("caddy")
    .or(Err(String::from("Can't find caddy binary")))?
    .args(&[
      "reload",
      "--config",
      caddyfile_path.as_os_str().to_str().unwrap(),
    ])
    .spawn()
    .map_err(|err| format!("Error reloading caddy {:?}", err))?;

  Ok(())
}

pub const LAUNCHER_ENV_URL: &str = ".launcher-env.json";

pub fn build_caddyfile_contents(
  caddy_admin_port: u16,
  conductor_admin_port: u16,
  conductor_app_interface_port: u16,
  web_apps_config: RunningWebApps,
) -> String {
  let mut caddyfile = format!(
    r#"{{
    admin localhost:{}
  }}
  "#,
    caddy_admin_port
  );

  for (holochain_version, holochain_web_apps) in web_apps_config {
    let admin_port = conductor_admin_port;
    let app_interface_port = conductor_app_interface_port;

    for (holochain_version, web_app_config) in web_apps_config {
      for (app_id, config) in web_app_config {
        let app_ui_port = config.app_ui_port;
        let web_app_files_path = config.path_to_web_app;

        caddyfile = format!(
          "{}

{}",
          caddyfile,
          caddyfile_config_for_app(
            admin_port,
            app_interface_port,
            &app_id,
            app_ui_port,
            web_app_files_path
          )
        );
      }
    }
  }

  caddyfile
}

fn caddyfile_config_for_app(
  admin_port: u16,
  app_interface_port: u16,
  app_id: &String,
  app_ui_port: u16,
  web_app_files_path: PathBuf,
) -> String {
  format!(
    r#":{} {{
      handle_path /{} {{
              respond 200 {{
                      body `{{
                              "APP_INTERFACE_PORT": {},
                              "ADMIN_INTERFACE_PORT": {},
                              "INSTALLED_APP_ID": "{}"
                      }}`
                      close
              }}
      }}

      header Cache-Control no-cache, no-store

      handle {{
              root * "{}"
              try_files {{path}} {{file}} /index.html
              file_server
      }}
}}
"#,
    app_ui_port,
    LAUNCHER_ENV_URL,
    app_interface_port,
    admin_port,
    app_id.clone(),
    web_app_files_path.into_os_string().to_str().unwrap(),
  )
}
