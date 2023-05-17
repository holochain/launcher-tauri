#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use file_system::{profile_holochain_data_dir, profile_tauri_dir};
use futures::lock::Mutex;
use hdk::prelude::AgentPubKey;
use launcher::error::LauncherError;
use running_state::RunningState;
use std::collections::HashMap;
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
use tauri::window::WindowBuilder;

mod commands;
mod file_system;
mod launcher;
mod menu;
mod running_state;
mod setup;
mod system_tray;


use crate::commands::choose_version::get_supported_versions;
use crate::commands::open_app::report_issue_cmd;
use crate::commands::save_app::{save_app, fetch_and_save_app};
// use crate::commands::start_app::start_app;
use crate::commands::restart::restart;
use crate::commands::quit::quit;
use crate::commands::{
  choose_version::choose_version_for_hdk,
  config::write_config,
  enable_app::{disable_app, enable_app, delete_clone},
  factory_reset::execute_factory_reset,
  get_app_info::get_app_info,
  icon_src::{get_icon_src, store_icon_src},
  get_state_info::get_state_info,
  install_app::install_app,
  install_devhub::install_devhub,
  open_app::open_app_ui,
  password::{initialize_keystore, unlock_and_launch},
  uninstall_app::uninstall_app,
  sign_zome_call::sign_zome_call,
  storage::get_storage_info,
  update_default_ui::update_default_ui,
};
use crate::launcher::manager::LauncherManager;
use crate::launcher::state::LauncherState;
use crate::menu::build_menu;
use crate::menu::handle_menu_event;
use crate::setup::logs::setup_logs;
use crate::system_tray::handle_system_tray_event;
use crate::file_system::Profile;

pub type BootstrapServerUrl = Option<String>;
pub type SignalingServerUrl = Option<String>;



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
      get_icon_src,
      store_icon_src,
      get_state_info,
      get_storage_info,
      open_app_ui,
      initialize_keystore,
      report_issue_cmd,
      unlock_and_launch,
      install_app,
      install_devhub,
      save_app,
      fetch_and_save_app,
      enable_app,
      disable_app,
      delete_clone,
      choose_version_for_hdk,
      get_supported_versions,
      uninstall_app,
      get_app_info,
      holochain_launcher_utils::shared_commands::open_url_cmd,
      // start_app,
      execute_factory_reset,
      restart,
      quit,
      write_config,
      sign_zome_call,
      update_default_ui,
      setup::logs::log,
    ])
    .setup(|app| {

      // reading profile from cli
      let cli_matches = app.get_cli_matches()?;
      let profile: Profile = match cli_matches.args.get("profile") {
        Some(data) => match data.value.clone() {
          Value::String(profile) => {
            if profile == "default" {
              eprintln!("Error: The name 'default' is not allowed for a profile.");
              panic!("Error: The name 'default' is not allowed for a profile.");
            }
            // \, /, and ? have a meaning as path symbols or domain socket url symbols and are therefore not allowed
            // because they would break stuff
            if profile.contains("/") || profile.contains("\\") || profile.contains("?") {
              eprintln!("Error: \"/\", \"\\\" and \"?\" are not allowed in profile names.");
              panic!("Error: \"/\", \"\\\" and \"?\" are not allowed in profile names.");
            }
            profile
          },
          _ => {
            // println!("ERROR: Value passed to --profile option could not be interpreted as string.");
            String::from("default")
            // panic!("Value passed to --profile option could not be interpreted as string.")
          }
        },
        None => String::from("default")
      };

      println!("Selected profile: {:?}", profile);

      let local_storage_path = profile_tauri_dir(profile.clone())?;

      app.manage(profile.clone());

      if let Err(err) = setup_logs(profile.clone()) {
        println!("Error setting up the logs: {:?}", err);
      }

      let _admin_window = WindowBuilder::new(
        app,
        "admin",
        tauri::WindowUrl::App("index.html".into())
      )
        .inner_size(1200.0, 880.0)
        .data_directory(local_storage_path)
        .resizable(true)
        .fullscreen(false)
        .title("Holochain Launcher")
        .center()
        .initialization_script("window.__HC_LAUNCHER_ENV__ = {}")
        .build()?;

      // manage the state of pubkeys associated to tauri windows. The keys of this hashmap are window labels
      let pubkey_map: Arc<Mutex<HashMap<String, AgentPubKey>>> = Arc::new(Mutex::new(HashMap::new()));
      app.manage(pubkey_map);

      let handle = Arc::new(app.handle());
      let launcher_state =
        tauri::async_runtime::block_on(async move { launch_manager(handle, profile).await });

      app.manage(Arc::new(Mutex::new(launcher_state)));

      Ok(())
    })
    .build(tauri::generate_context!());

  match builder_result {
    Ok(builder) => {
      builder.run(|_app_handle, event| {
        // This event is emitted upon quitting the Launcher via cmq+Q on macOS.
        // Sidecar binaries need to get explicitly killed in this case (https://github.com/holochain/launcher/issues/141)
        if let RunEvent::Exit = event {
          tauri::api::process::kill_children();
        }
        // This event is emitted upon pressing the x to close the Launcher admin window
        // The app is prevented from exiting to keep it running in the background with the system tray
        if let RunEvent::ExitRequested { api, .. } = event {
          api.prevent_exit();
        }
      });
    }
    Err(err) => log::error!("Error building the app: {:?}", err),
  }
}

async fn launch_manager(app_handle: Arc<AppHandle>, profile: Profile) -> RunningState<LauncherManager, LauncherError> {
  let holochain_dir = match profile_holochain_data_dir(profile.clone()) {
    Ok(dir) => dir,
    Err(e) => {
      log::error!("Failed to get holochain data dir when trying to launch the LauncherManager: {}", e);
      return RunningState::Error(e)
    }
  };
  if Path::new(&holochain_dir.join("conductor")).exists() {
    return RunningState::Error(LauncherError::OldFilesExist);
  }

  let manager_launch = LauncherManager::launch(app_handle, profile).await;

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
