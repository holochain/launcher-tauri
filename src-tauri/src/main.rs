#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use file_system::root_holochain_data_path;
use futures::lock::Mutex;
use launcher::error::LauncherError;
use running_state::RunningState;
use std::path::Path;
use std::sync::Arc;
use tauri::AppHandle;
use serde_json::value::Value;

use system_tray::initial_system_tray;
use tauri;
use tauri::api::process::kill_children;
use tauri::Manager;
use tauri::RunEvent;
use tauri::SystemTray;
use tauri::SystemTrayEvent;

mod commands;
mod file_system;
mod launcher;
mod menu;
mod running_state;
mod setup;
mod system_tray;

use crate::commands::choose_version::get_supported_versions;
use crate::commands::open_app::open_url_cmd;
use crate::commands::open_app::report_issue_cmd;
use crate::commands::save_app::save_app;
use crate::commands::start_app::start_app;
use crate::commands::restart::restart;
use crate::commands::quit::quit;
use crate::commands::{
  choose_version::choose_version_for_hdk,
  config::write_config,
  enable_app::{disable_app, enable_app},
  factory_reset::execute_factory_reset,
  get_app_info::get_app_info,
  get_state_info::get_state_info,
  install_app::install_app,
  open_app::open_app_ui,
  password::{initialize_keystore, unlock_and_launch},
  uninstall_app::uninstall_app,
  storage::get_storage_info,
};
use crate::launcher::manager::LauncherManager;
use crate::launcher::state::LauncherState;
use crate::menu::build_menu;
use crate::menu::handle_menu_event;
use crate::setup::logs::setup_logs;
use crate::system_tray::handle_system_tray_event;
use crate::file_system::CustomPath;



fn main() {


  let builder_result = tauri::Builder::default()
    .menu(build_menu())
    .on_menu_event(|event| handle_menu_event(event.menu_item_id(), event.window()))
    .system_tray(SystemTray::new().with_menu(initial_system_tray()))
    .on_system_tray_event(|app, event| match event {
      SystemTrayEvent::MenuItemClick { id, .. } => handle_system_tray_event(app, id),
      _ => {}
    })
    .invoke_handler(tauri::generate_handler![
      get_state_info,
      get_storage_info,
      open_app_ui,
      initialize_keystore,
      report_issue_cmd,
      unlock_and_launch,
      install_app,
      save_app,
      enable_app,
      disable_app,
      choose_version_for_hdk,
      get_supported_versions,
      uninstall_app,
      get_app_info,
      open_url_cmd,
      start_app,
      execute_factory_reset,
      restart,
      quit,
      write_config,
      setup::logs::log,
    ])
    .setup(|app| {

      // reading custom path from cli
      let cli_matches = app.get_cli_matches()?;
      let custom_path: CustomPath = match cli_matches.args.get("config-path") {
        Some(data) => match data.value.clone() {
          Value::String(path) => CustomPath {
            custom_path: Some(path),
          },
          _ => {
            // println!("ERROR: Value passed to --config-path option could not be interpreted as string.");
            CustomPath {
              custom_path: None
            }
            // panic!("Value passed to --config-path option could not be interpreted as string.")
          }
        },
        None => CustomPath {
          custom_path: None
        },
      };

      app.manage(custom_path.clone());

      if let Err(err) = setup_logs(custom_path.custom_path.clone()) {
        println!("Error setting up the logs: {:?}", err);
      }

      let handle = app.handle().clone();
      let launcher_state =
        tauri::async_runtime::block_on(async move { launch_manager(handle, custom_path.custom_path).await });

      app.manage(Arc::new(Mutex::new(launcher_state)));

      Ok(())
    })
    .build(tauri::generate_context!());

  match builder_result {
    Ok(builder) => {
      builder.run(|_app_handle, event| {
        if let RunEvent::ExitRequested { api, .. } = event {
          api.prevent_exit();
        }
      });
    }
    Err(err) => log::error!("Error building the app: {:?}", err),
  }
}

async fn launch_manager(app_handle: AppHandle, custom_path: Option<String>) -> RunningState<LauncherManager, LauncherError> {
  if Path::new(&root_holochain_data_path(custom_path.clone()).join("conductor")).exists() {
    return RunningState::Error(LauncherError::OldFilesExist);
  }

  let manager_launch = LauncherManager::launch(app_handle, custom_path).await;

  match manager_launch {
    Ok(launcher_manager) => {
      log::info!("Launch setup successful");
      RunningState::Running(launcher_manager)
    }
    Err(error) => {
      kill_children();
      log::error!("There was an error launching holochain: {:?}", error);
      RunningState::Error(error)
    }
  }
}
