#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use std::sync::Arc;
use std::sync::Mutex;

use portpicker;
use tauri;
use tauri::api::process::kill_children;
use tauri::Event;
use tauri::SystemTrayEvent;

mod commands;
mod launch;
mod menu;
mod setup;
mod state;
mod system_tray;
mod uis;

use crate::commands::{
  enable_app::{disable_app, enable_app},
  factory_reset::execute_factory_reset,
  get_connection_status::get_connection_status,
  get_web_app_info::get_web_app_info,
  install_app::install_app,
  open_app::open_app_ui,
  uninstall_app::uninstall_app,
};
use crate::menu::build_menu;
use crate::menu::handle_menu_event;
use crate::setup::is_holochain_already_running;
use crate::setup::logs::setup_logs;
use crate::state::ConnectionStatus;
use crate::state::LauncherState;
use crate::state::RunningPorts;
use crate::system_tray::build_system_tray;
use crate::system_tray::handle_system_tray_event;

fn main() {
  if let Err(err) = setup_logs() {
    println!("Error setting up the logs: {:?}", err);
  }

  let already_running =
    tauri::async_runtime::block_on(async move { is_holochain_already_running().await });

  // If holochain is already running, only display a small notice window
  if already_running {
    let build_result = tauri::Builder::default()
      .manage(LauncherState {
        connection_status: Arc::new(Mutex::new(ConnectionStatus::AlreadyRunning)),
      })
      .invoke_handler(tauri::generate_handler![get_connection_status])
      .run(tauri::generate_context!());
    if let Err(err) = build_result {
      log::error!("Error building the window: {}", err);
    }
    return ();
  }

  let free_port = portpicker::pick_unused_port().expect("No ports free");
  let caddy_free_port = portpicker::pick_unused_port().expect("No ports free");
  let ports = RunningPorts {
    admin_interface_port: free_port,
    caddy_admin_port: caddy_free_port,
  };
  let cloned_ports = ports.clone();
  let launch_result =
    tauri::async_runtime::block_on(
      async move { launch::launch_children_processes(cloned_ports).await },
    );

  let state = match launch_result {
    Ok(()) => {
      log::info!("Launch setup successful");
      LauncherState {
        connection_status: Arc::new(Mutex::new(ConnectionStatus::Connected(ports))),
      }
    }
    Err(err) => {
      kill_children();
      log::error!("There was an error launching holochain: {:?}", err);
      LauncherState {
        connection_status: Arc::new(Mutex::new(ConnectionStatus::Error { error: err })),
      }
    }
  };

  let builder_result = tauri::Builder::default()
    .manage(state)
    .menu(build_menu())
    .on_menu_event(|event| handle_menu_event(event.menu_item_id(), event.window()))
    .system_tray(build_system_tray())
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => handle_system_tray_event(app, id),
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![
      get_connection_status,
      open_app_ui,
      install_app,
      enable_app,
      disable_app,
      uninstall_app,
      get_web_app_info,
      execute_factory_reset,
      setup::logs::log,
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
