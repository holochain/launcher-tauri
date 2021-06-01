#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::process::Command;
use std::{thread, time::Duration};

mod config;

fn main() {
  config::create_initial_config_if_necessary();

  Command::new("lair-keystore")
    .arg(format!(
      "-d {}",
      config::keystore_data_path()
        .into_os_string()
        .to_str()
        .unwrap()
    ))
    .spawn()
    .expect("failed to execute process");

  thread::sleep(Duration::from_millis(1000));

  Command::new("holochain")
    .arg(format!(
      "-c {}",
      config::conductor_config_path()
        .into_os_string()
        .to_str()
        .unwrap()
    ))
    .spawn()
    .expect("failed to execute process");

  tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
