use lair_keystore_manager::error::LaunchTauriSidecarError;
use std::path::PathBuf;
use tauri::api::process::{Command, CommandEvent};

use crate::running_apps::{AppUiInfo, RunningApps};

pub fn launch_caddy_process(caddyfile_path: PathBuf) -> Result<(), LaunchTauriSidecarError> {
  let (mut caddy_rx, _) = Command::new_sidecar("caddy")
    .or(Err(LaunchTauriSidecarError::BinaryNotFound))?
    .args(&[
      "run",
      "--config",
      caddyfile_path.as_os_str().to_str().unwrap(),
    ])
    .spawn()
    .map_err(|err| LaunchTauriSidecarError::FailedToExecute(format!("{:?}", err)))?;

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

pub fn reload_caddy(caddyfile_path: PathBuf) -> Result<(), LaunchTauriSidecarError> {
  Command::new_sidecar("caddy")
    .or(Err(LaunchTauriSidecarError::BinaryNotFound))?
    .args(&[
      "reload",
      "--config",
      caddyfile_path.as_os_str().to_str().unwrap(),
    ])
    .spawn()
    .map_err(|err| LaunchTauriSidecarError::FailedToExecute(format!("{:?}", err)))?;

  Ok(())
}

pub const LAUNCHER_ENV_URL: &str = ".launcher-env.json";

pub fn build_caddyfile_contents(
  caddy_admin_port: u16,
  conductor_admin_port: u16,
  conductor_app_interface_port: u16,
  running_apps: &RunningApps,
) -> String {
  let mut caddyfile = format!(
    r#"{{
    admin localhost:{}
  }}
  "#,
    caddy_admin_port
  );

  for (app_id, app_ui_info) in running_apps {
    if let AppUiInfo::WebApp(web_app_info) = app_ui_info {
      let app_ui_port = web_app_info.app_ui_port;
      let web_app_files_path = web_app_info.path_to_web_app.clone();

      caddyfile = format!(
        "{}

        {}",
        caddyfile,
        caddyfile_config_for_app(
          conductor_admin_port,
          conductor_app_interface_port,
          &app_id,
          app_ui_port,
          web_app_files_path
        )
      );
    }
  }

  caddyfile
}

fn caddyfile_config_for_app(
  conductor_admin_port: u16,
  conductor_app_interface_port: u16,
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
    conductor_app_interface_port,
    conductor_admin_port,
    app_id.clone(),
    web_app_files_path.into_os_string().to_str().unwrap(),
  )
}
