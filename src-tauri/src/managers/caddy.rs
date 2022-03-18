use std::fs;

use tauri::api::process::{Command, CommandEvent};

use super::file_system::FileSystemManager;

pub struct CaddyManager {
  caddy_admin_port: u16,
}

impl CaddyManager {
  pub async fn launch() -> Result<CaddyManager, String> {
    let caddy_admin_port = portpicker::pick_unused_port().expect("No ports free");

    let manager = CaddyManager { caddy_admin_port };

    launch_caddy()?;

    Ok(manager)
  }

  fn initial_caddyfile(&self) -> String {
    format!(
      r#"{{
    admin localhost:{}
  }}
  "#,
      self.caddy_admin_port
    )
  }

  pub fn write_caddyfile(&self, new_content: String) -> Result<(), String> {
    let content = format!(
      "{}
{}",
      self.initial_caddyfile(),
      new_content
    );

    fs::write(FileSystemManager::caddyfile_path(), content)
      .map_err(|err| format!("Error writing Caddyfile: {:?}", err))
  }

  /// Refreshes the running apps and reloads caddy to be consistent with them
  /// Execute this when there has been some change in the status of an app (enabled, disabled, uninstalled...)
  pub fn reload_caddy() -> Result<(), String> {
    log::info!("Reloading Caddy");

    Command::new_sidecar("caddy")
      .or(Err(String::from("Can't find caddy binary")))?
      .args(&[
        "reload",
        "--config",
        FileSystemManager::caddyfile_path()
          .as_os_str()
          .to_str()
          .unwrap(),
      ])
      .spawn()
      .map_err(|err| format!("Error reloading caddy {:?}", err))?;

    Ok(())
  }
}

/// Launches caddy, execute only on launcher start
fn launch_caddy() -> Result<(), String> {
  let (mut caddy_rx, _) = Command::new_sidecar("caddy")
    .or(Err(String::from("Can't find caddy binary")))?
    .args(&[
      "run",
      "--config",
      FileSystemManager::caddyfile_path()
        .as_os_str()
        .to_str()
        .unwrap(),
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
