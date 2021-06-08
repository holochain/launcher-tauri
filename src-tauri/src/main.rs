#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use holochain_conductor_api_rust::AdminWebsocket;
use std::{process::Command, thread, time::Duration};
use tauri;

mod config;
mod uis;

use crate::config::admin_url;
use crate::uis::{install::install_ui, port_mapping::get_port_mapping};

#[tokio::main]
async fn main() {
  match launch().await {
    Ok(()) => (),
    Err(err) => println!("There was an error launching holochain: {}", err),
  }
}

async fn launch() -> Result<(), String> {
  config::create_initial_config_if_necessary();

  Command::new("lair-keystore")
    .arg("-d")
    .arg(
      config::keystore_data_path()
        .into_os_string()
        .to_str()
        .unwrap(),
    )
    .spawn()
    .expect("failed to execute process");

  thread::sleep(Duration::from_millis(1000));

  Command::new("holochain")
    .arg("-c")
    .arg(
      config::conductor_config_path()
        .into_os_string()
        .to_str()
        .unwrap(),
    )
    .spawn()
    .expect("failed to execute process");

  let mut ws = AdminWebsocket::connect(admin_url())
    .await
    .or(Err(String::from("Could not connect to conductor")))?;

  uis::activate::activate_uis_for_active_apps(&mut ws).await?;

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_port_mapping, install_ui])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  Ok(())
}
