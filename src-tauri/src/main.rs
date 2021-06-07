#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::{process::Command, thread, time::Duration};
use tauri;

mod config;
mod install_ui;

fn main() {
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

  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![install_ui::install_ui])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
