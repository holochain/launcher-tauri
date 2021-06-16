#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::{process::Command, thread, time::Duration};
use tauri::Manager;
use tauri::State;
use tauri::{self, WindowEvent};
use tauri::{CustomMenuItem, SystemTrayMenuItem};

mod config;
mod setup;
mod state;
mod uis;

use crate::setup::setup_conductor;
use crate::state::{get_logs, HolochainLauncherState};
use crate::uis::{install::install_ui, open::open_app_ui};

#[tokio::main]
async fn main() {
  let mut launcher_state = HolochainLauncherState {
    child_processes: Arc::new(Mutex::new(vec![])),
    logs: Arc::new(Mutex::new(HashMap::new())),
  };

  match launch_holochain(&mut launcher_state).await {
    Ok(()) => (),
    Err(err) => {
      launcher_state.log(format!("There was an error launching holochain: {:?}", err));
      launcher_state.terminate_all_children();
    }
  }

  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let show_admin = CustomMenuItem::new("show_admin".to_string(), "Show Admin");
  let hide_admin = CustomMenuItem::new("hide_admin".to_string(), "Hide Admin");

  let tray_menu_items = vec![
    SystemTrayMenuItem::Custom(show_admin),
    SystemTrayMenuItem::Custom(hide_admin),
    SystemTrayMenuItem::Separator,
    SystemTrayMenuItem::Custom(quit),
  ];

  tauri::Builder::default()
    .manage(launcher_state.clone())
    .system_tray(tray_menu_items)
    .on_window_event(move |event| match event.event() {
      WindowEvent::Destroyed | WindowEvent::CloseRequested => {
        launcher_state.terminate_all_children();

        std::process::exit(0);
      }
      _ => {}
    })
    .on_system_tray_event(move |app, event| match event.menu_item_id().as_str() {
      "quit" => {
        let state: State<HolochainLauncherState> = app.state();

        state.inner().terminate_all_children();

        std::process::exit(0);
      }
      "show_admin" => {
        if let Err(err) = app.get_window("admin").unwrap().show() {
          let state: State<HolochainLauncherState> = app.state();

          state
            .inner()
            .log(format!("Error trying to show the admin: {:?}", err));
        }
      }
      "hide_admin" => {
        if let Err(err) = app.get_window("admin").unwrap().hide() {
          let state: State<HolochainLauncherState> = app.state();

          state
            .inner()
            .log(format!("Error trying to show the admin: {:?}", err));
        }
      }
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![open_app_ui, install_ui, get_logs])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

async fn launch_holochain(launcher_state: &mut HolochainLauncherState) -> Result<(), String> {
  config::create_initial_config_if_necessary();

  let mut lair_child = Command::new("lair-keystore")
    .arg("-d")
    .arg(
      config::keystore_data_path()
        .into_os_string()
        .to_str()
        .unwrap(),
    )
    .spawn()
    .map_err(|err| format!("Failed to execute lair-keystore: {:?}", err))?;

  thread::sleep(Duration::from_millis(1000));

  if let Ok(Some(_)) = lair_child.try_wait() {
    return Err(String::from(
      "Failed to execute lair: clean the lair directory and try again",
    ));
  }
  launcher_state.add_child_process(lair_child);

  let mut holochain_child = Command::new("holochain")
    .arg("-c")
    .arg(
      config::conductor_config_path()
        .into_os_string()
        .to_str()
        .unwrap(),
    )
    .spawn()
    .map_err(|err| format!("Failed to execute holochain: {:?}", err))?;

  thread::sleep(Duration::from_millis(1000));

  if let Ok(Some(_)) = holochain_child.try_wait() {
    return Err(String::from(
      "Failed to execute holochain: do you have anything running on ports 8888 or 8889?",
    ));
  }

  launcher_state.add_child_process(holochain_child);

  setup_conductor(&launcher_state).await?;

  Ok(())
}
