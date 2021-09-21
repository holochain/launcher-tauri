use std::{thread, time::Duration};

use crate::config::{admin_url, DEFAULT_APP_PORT};
use holochain_conductor_client::AdminWebsocket;
use tauri::api::process::Command;

use crate::{config, uis::caddy};

pub async fn launch_children_processes() -> Result<(), String> {
  config::create_initial_config_if_necessary();

  Command::new_sidecar("lair-keystore")
    .or(Err(String::from("Can't find lair-keystore binary")))?
    .args(&[
      "-d",
      config::keystore_data_path()
        .into_os_string()
        .to_str()
        .unwrap(),
    ])
    .spawn()
    .map_err(|err| format!("Failed to execute lair-keystore: {:?}", err))?;

  log::info!("Launched lair-keystore");

  thread::sleep(Duration::from_millis(1000));

  Command::new_sidecar("holochain")
    .or(Err(String::from("Can't find holochain binary")))?
    .args(&[
      "-c",
      config::conductor_config_path()
        .into_os_string()
        .to_str()
        .unwrap(),
    ])
    .spawn()
    .map_err(|err| format!("Failed to execute holochain: {:?}", err))?;

  log::info!("Launched holochain");

  thread::sleep(Duration::from_millis(1000));

  setup_conductor().await?;

  caddy::launch_caddy().await?;

  Ok(())
}

async fn setup_conductor() -> Result<(), String> {
  let mut ws = AdminWebsocket::connect(admin_url())
    .await
    .or(Err(String::from("Could not connect to conductor")))?;

  log::info!("Connected to admin conductor");

  let app_interfaces = ws
    .list_app_interfaces()
    .await
    .or(Err(String::from("Could not list app interfaces")))?;

  if !app_interfaces.contains(&DEFAULT_APP_PORT) {
    ws.attach_app_interface(DEFAULT_APP_PORT)
      .await
      .or(Err(String::from("Could not attach app interface")))?;
    log::info!("Attached app interface to {}", DEFAULT_APP_PORT);
  }

  Ok(())
}
