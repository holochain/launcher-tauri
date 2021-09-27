#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use tauri;
use tauri::api::process::kill_children;
use tauri::SystemTrayEvent;
use tauri::{Event};
use portpicker;

mod commands;
mod launch;
mod setup;
mod menu;
mod system_tray;
mod uis;
mod state;

use crate::commands::{
  enable_app::{disable_app, enable_app},
  get_web_app_info::get_web_app_info,
  install_app::install_app,
  open_app::open_app_ui,
  uninstall_app::uninstall_app,
  factory_reset::execute_factory_reset,
  get_admin_port::get_admin_port
};
use crate::setup::logs::setup_logs;
use crate::menu::build_menu;
use crate::menu::handle_menu_event;
use crate::system_tray::build_system_tray;
use crate::system_tray::handle_system_tray_event;
use crate::state::LauncherState;

fn main() {
  if let Err(err) = setup_logs() {
    println!("Error setting up the logs: {:?}", err);
  }

  let free_port = portpicker::pick_unused_port().expect("No ports free");

  let builder_result = tauri::Builder::default()
  .manage(LauncherState {
    admin_interface_port: free_port
  })
    .menu(build_menu())
    .on_menu_event(|event| handle_menu_event(event.menu_item_id(), event.window()))
    .system_tray(build_system_tray())
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => handle_system_tray_event(app, id),
      _ => {}
    })
    .setup(move |_app| {
      tauri::async_runtime::block_on(async move {
        match launch::launch_children_processes(free_port).await {
          Ok(()) => {
            log::info!("Launch setup successful");
          }
          Err(err) => {
            kill_children();
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
      get_web_app_info,
      get_admin_port,
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
