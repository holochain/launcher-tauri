#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::{process::Command, thread, time::Duration};
use tauri;

mod config;
mod setup;
mod uis;

use crate::setup::setup_conductor;
use crate::uis::{install::install_ui, launch::launch_app_ui};

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

  setup_conductor().await?;

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![launch_app_ui, install_ui])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  Ok(())
}
