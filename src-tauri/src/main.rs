#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::process::Child;
use std::sync::{Arc, Mutex};
use std::{process::Command, thread, time::Duration};
use tauri;
use tauri::Manager;
use tauri::State;
use tauri::{CustomMenuItem, SystemTrayMenuItem};

mod config;
mod setup;
mod state;
mod uis;

use crate::setup::setup_conductor;
use crate::state::HolochainLauncherState;
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

  let launcher_state = HolochainLauncherState {
    child_processes: Arc::new(Mutex::new(vec![])),
  };

  let lair_child = Command::new("lair-keystore")
    .arg("-d")
    .arg(
      config::keystore_data_path()
        .into_os_string()
        .to_str()
        .unwrap(),
    )
    .spawn()
    .expect("failed to execute process");
  launcher_state
    .child_processes
    .lock()
    .unwrap()
    .push(lair_child);

  thread::sleep(Duration::from_millis(1000));

  let holochain_child = Command::new("holochain")
    .arg("-c")
    .arg(
      config::conductor_config_path()
        .into_os_string()
        .to_str()
        .unwrap(),
    )
    .spawn()
    .expect("failed to execute process");
  launcher_state
    .child_processes
    .lock()
    .unwrap()
    .push(holochain_child);

  setup_conductor(&launcher_state).await?;

  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let tray_menu_items = vec![SystemTrayMenuItem::Custom(quit)];

  tauri::Builder::default()
    .manage(launcher_state)
    .system_tray(tray_menu_items)
    .on_system_tray_event(move |app, event| match event.menu_item_id().as_str() {
      "quit" => {
        let state: State<HolochainLauncherState> = app.state();

        let mut inner_state = state.inner().child_processes.lock().unwrap();
        let child_processes: &mut Vec<Child> = inner_state.as_mut();
        for child_process in child_processes.into_iter() {
          child_process.kill();
        }
        std::process::exit(0);
      }
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![launch_app_ui, install_ui])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  Ok(())
}
