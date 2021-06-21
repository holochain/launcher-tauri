#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::{thread, time::Duration};
use tauri;
use tauri::api::process::Command;
use tauri::Manager;
use tauri::SystemTray;
use tauri::SystemTrayEvent;
use tauri::SystemTrayMenu;
use tauri::WindowBuilder;
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
  let show_admin = CustomMenuItem::new("show_admin".to_string(), "Show Admin");

  let sys_tray_menu = SystemTrayMenu::new()
    .add_item(show_admin)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit);

  let sys_tray = SystemTray::new().with_menu(sys_tray_menu);

  tauri::Builder::default()
    .manage(launcher_state.clone())
    .system_tray(sys_tray)
    .on_system_tray_event(move |app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
        "quit" => {
          // Closing all the windows exits the app and kills all the children processes
          for window in app.windows().values() {
            let _r = window.close();
          }
        }
        "show_admin" => {
          let admin_window = app.get_window("admin");

          if let Some(window) = admin_window {
            let _r = window.set_focus();
          } else {
            // Window was closed: we need to recreate it
            let _r = app.create_window(
              "admin".into(),
              WindowUrl::App("index.html".into()),
              move |window_builder, webview_attributes| {
                (window_builder.title("Holochain Admin"), webview_attributes)
              },
            );
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

  println!("Launched lair-keystore");

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
  println!("Launched holochain");

  thread::sleep(Duration::from_millis(1000));

  setup_conductor().await?;

  Ok(())
}
