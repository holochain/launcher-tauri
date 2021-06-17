#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::{thread, time::Duration};
use tauri;
use tauri::api::process::Command;
use tauri::Icon;
use tauri::Manager;
use tauri::State;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::SystemTrayMenu;
use tauri::WindowUrl;
use tauri::{CustomMenuItem, SystemTrayMenuItem};

mod config;
mod setup;
mod state;
mod uis;

use crate::setup::setup_conductor;
use crate::state::{get_logs, HolochainLauncherState};
use crate::uis::{
  activate::activate_app_ui, deactivate::deactivate_app_ui, install::install_ui, open::open_app_ui,
};

#[tokio::main]
async fn main() {
  let launcher_state = HolochainLauncherState::new();

  match launch_holochain().await {
    Ok(()) => (),
    Err(err) => {
      launcher_state.log(format!("There was an error launching holochain: {:?}", err));
    }
  }

  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let show_hide_admin = CustomMenuItem::new("show_admin".to_string(), "Show/Hide Admin");

  let sys_tray_menu = SystemTrayMenu::new()
    .add_item(show_hide_admin)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit);

  let sys_tray = SystemTray::new()
    .with_icon(Icon::File("icons/holochain.png".into()))
    .with_menu(sys_tray_menu);

  tauri::Builder::default()
    .manage(launcher_state.clone())
    .system_tray(sys_tray)
    .on_system_tray_event(move |app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
        "quit" => {
          let admin_window = app.get_window("admin");
          if let Some(window) = admin_window {
            println!("hii1");
            let _r = window.close();
          }

          let background_window = app.get_window("background");
          if let Some(window) = background_window {
            println!("hii2");
            let _r = window.close();
          }
        }
        "show_hide_admin" => {
          let admin_window = app.get_window("admin");

          if let Some(window) = admin_window {
            if window.is_visible().unwrap() {
              let _r = window.hide();
            } else {
              let _r = window.show();
            }
          } else {
            // Window was closed: we need to recreate it
            app.create_window("admin".into(), WindowUrl::new("index.html"), |w, w2| {
              (w, w2)
            });
          }
        }
        _ => {}
      },
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![
      open_app_ui,
      install_ui,
      get_logs,
      deactivate_app_ui,
      activate_app_ui
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

async fn launch_holochain() -> Result<(), String> {
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

  thread::sleep(Duration::from_millis(1000));

  setup_conductor().await?;

  Ok(())
}
