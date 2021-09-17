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
use tauri::{CustomMenuItem, Event, SystemTrayMenuItem};

mod commands;
mod config;
mod logs;
mod setup;
mod uis;

use crate::commands::{
  enable_app::{disable_app, enable_app},
  get_slots_to_configure::get_slots_to_configure,
  install_app::install_app,
  open_app::open_app_ui,
  uninstall_app::uninstall_app,
};
use crate::logs::setup_logs;
use crate::setup::setup_conductor;
use crate::uis::caddy;

fn main() {
  if let Err(err) = setup_logs() {
    println!("Error setting up the logs: {:?}", err);
  }

  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let show_admin = CustomMenuItem::new("show_admin".to_string(), "Show Admin");
  let open_logs = CustomMenuItem::new("open_logs".to_string(), "Open Logs");

  let sys_tray_menu = SystemTrayMenu::new()
    .add_item(open_logs)
    .add_item(show_admin)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit);

  let sys_tray = SystemTray::new().with_menu(sys_tray_menu);

  let builder_result = tauri::Builder::default()
    .system_tray(sys_tray)
    .on_system_tray_event(|app, event| {
      match event {
        SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
          "quit" => {
            app.exit(0);
          }
          "show_admin" => {
            let admin_window = app.get_window("admin");

            if let Some(window) = admin_window {
              window.show().unwrap();
              window.set_focus().unwrap();
            } else {
              // Window was closed: we need to recreate it
              let _r = app.create_window(
                "admin",
                WindowUrl::App("index.html".into()),
                move |window_builder, webview_attributes| {
                  (window_builder.title("Holochain Admin"), webview_attributes)
                },
              );
              log::info!("Creating admin window {:?}", _r);
            }
          }
          "open_logs" => {
            logs::open_logs();
          }
          _ => {}
        },
        _ => {}
      }
    })
    .setup(|_app| {
      tauri::async_runtime::block_on(async move {
        match launch_children_processes().await {
          Ok(()) => {
            log::info!("Launch setup successful");
          }
          Err(err) => {
            log::error!("There was an error launching holochain: {:?}", err);
          }
        }
      });
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      open_app_ui,
      install_app,
      enable_app,
      disable_app,
      uninstall_app,
      get_slots_to_configure,
      logs::log,
    ])
    .build(tauri::generate_context!());

  match builder_result {
    Ok(builder) => {
      builder.run(|_app_handle, event| {
        if let Event::ExitRequested { api, .. } = event {
          api.prevent_exit();
        }
      });
    }
    Err(err) => log::error!("Error building the app: {:?}", err),
  }
}

async fn launch_children_processes() -> Result<(), String> {
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
