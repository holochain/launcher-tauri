use holochain_versions::HolochainVersion;
use std::{collections::HashMap, path::PathBuf};
use tauri::{
  api::process::{Command, CommandEvent},
  async_runtime::Receiver,
};

pub fn launch_caddy_process(caddyfile_path: PathBuf) -> Result<Receiver<CommandEvent>, String> {
  let (mut caddy_rx, _) = Command::new_sidecar("caddy")
    .or(Err(String::from("Can't find caddy binary")))?
    .args(&[
      "run",
      "--config",
      caddyfile_path.as_os_str().to_str().unwrap(),
    ])
    .spawn()
    .map_err(|err| format!("Error running caddy {:?}", err))?;

  Ok(caddy_rx)
}

pub fn reload_caddy(caddyfile_path: PathBuf) -> Result<Receiver<CommandEvent>, String> {
  Command::new_sidecar("caddy")
    .or(Err(String::from("Can't find caddy binary")))?
    .args(&[
      "reload",
      "--config",
      caddyfile_path.as_os_str().to_str().unwrap(),
    ])
    .spawn()
    .map_err(|err| format!("Error reloading caddy {:?}", err))?;
}

pub const LAUNCHER_ENV_URL: &str = ".launcher-env.json";

pub struct HolochainWebAppsConfig {
  admin_port: u16,
  app_interface_port: u16,
  running_apps: RunningApps,
}

pub fn build_caddyfile_contents(
  caddy_admin_port: u16,
  web_apps_config: HashMap<HolochainVersion, HolochainWebAppsConfig>,
) -> String {
  let mut caddyfile = format!(
    r#"{{
    admin localhost:{}
  }}
  "#,
    caddy_admin_port
  );

  for (holochain_version, holochain_web_apps) in web_apps_config {
    let admin_port = holochain_web_apps.admin_port;
    let app_interface_port = holochain_web_apps.app_interface_port;

    for (app_id, app_config) in holochain_web_apps.web_apps_config {
      let app_ui_port = app_config.app_ui_port;
      let web_app_files_path = app_config.web_app_files_path;

      caddyfile = format!(
          "{}

{}",
          caddyfile,
          caddyfile_config_for_app(admin_port, app_interface_port, &app_id, app_ui_port, web_app_files_path);
      );
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
