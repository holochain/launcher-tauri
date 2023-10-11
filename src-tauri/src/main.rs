#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use file_system::{profile_holochain_data_dir, profile_tauri_dir};
use futures::lock::Mutex;
use hdk::prelude::AgentPubKey;
use launcher::error::LauncherError;
use running_state::RunningState;
use tauri::Window;
use tauri::WindowEvent;
use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;
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
use crate::commands::notifications::IconState;
use crate::commands::notifications::SysTrayIconState;
use crate::commands::open_app::report_issue_cmd;
use crate::commands::save_app::{save_app, fetch_and_save_app, fetch_gui};
// use crate::commands::start_app::start_app;
use crate::commands::restart::restart;
use crate::commands::quit::quit;
use crate::commands::{
  choose_version::choose_version_for_hdk,
  config::{write_config, get_default_bootstrap, get_default_signaling},
  enable_app::{disable_app, enable_app, delete_clone},
  factory_reset::execute_factory_reset,
  get_app_info::get_app_info,
  icon_src::{get_icon_src, store_icon_src},
  get_state_info::get_state_info,
  install_app::install_app,
  install_devhub::install_devhub,
  network_stats::dump_network_stats,
  notifications::{notify_os, notify_tauri, clear_happ_notifications, clear_systray_icon, reset_happ_notification_count},
  open_app::open_app_ui,
  password::{initialize_keystore, unlock_and_launch},
  uninstall_app::uninstall_app,
  sign_zome_call::sign_zome_call,
  storage::get_storage_info,
  update_default_ui::{fetch_and_update_default_gui, update_default_ui},
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
      choose_version_for_hdk,
      clear_happ_notifications,
      clear_systray_icon,
      disable_app,
      delete_clone,
      dump_network_stats,
      enable_app,
      execute_factory_reset,
      fetch_and_save_app,
      fetch_and_update_default_gui,
      fetch_gui,
      get_app_info,
      get_default_bootstrap,
      get_default_signaling,
      get_icon_src,
      get_state_info,
      get_storage_info,
      get_supported_versions,
      initialize_keystore,
      install_app,
      install_devhub,
      notify_os,
      notify_tauri,
      open_app_ui,
      holochain_launcher_utils::shared_commands::open_url_cmd,
      // start_app,
      quit,
      report_issue_cmd,
      reset_happ_notification_count,
      restart,
      save_app,
      sign_zome_call,
      store_icon_src,
      update_default_ui,
      uninstall_app,
      unlock_and_launch,
      write_config,
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

      // Only allow single-instance of the Launcher (https://github.com/holochain/launcher/issues/153), except
      // a special profile is specified via the CLI
      if profile == String::from("default") {
        app.handle().plugin(tauri_plugin_single_instance::init(move |app, argv, cwd| {
          println!("{}, {argv:?}, {cwd}", app.package_info().name);

          let admin_window = app.get_window("admin");
          // println!("admin window? {:?}", admin_window);
          if let Some(window) = admin_window {
            window.show().unwrap();
            window.unminimize().unwrap();
            window.set_focus().unwrap();
          } else {
            let local_storage_path = profile_tauri_dir(String::from("default")).unwrap();
            let r = build_admin_window(&app.app_handle(), local_storage_path).unwrap();
            log::info!("Creating admin window {:?}", r);
          }
        }))?;
      }

      println!("Selected profile: {:?}", profile);

      app.manage(profile.clone());

      app.manage(Mutex::new(SysTrayIconState { icon_state: IconState::Clean }));

      if let Err(err) = setup_logs(profile.clone()) {
        println!("Error setting up the logs: {:?}", err);
      }

      let local_storage_path = profile_tauri_dir(profile.clone())?;

      let _admin_window = build_admin_window(&app.app_handle(), local_storage_path)?;

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
      builder.run(|app_handle, event| {

        match event {
          // This event is emitted upon quitting the Launcher via cmq+Q on macOS.
          // Sidecar binaries need to get explicitly killed in this case (https://github.com/holochain/launcher/issues/141)
          RunEvent::Exit => tauri::api::process::kill_children(),

          // This event is emitted upon pressing the x to close the Launcher admin window
          // The app is prevented from exiting to keep it running in the background with the system tray
          RunEvent::ExitRequested { api, .. } => api.prevent_exit(),

          // If a window is requested to be closed, hide it instead. This is to keep the UI running in the
          // background to be able to send/receive notifications.
          // TODO garbage collect windows in the front-end if they have notificationSettings all turned off
          RunEvent::WindowEvent { label, event: window_event, .. } => {
            match window_event {
              // On macOS the window remains in the dock even when hidden, i.e. it needs to become visible again
              // upon clicking on the icon in the dock
              #[cfg(target_os = "macos")]
              WindowEvent::Focused(true) => {
                let window_option = app_handle.get_window(&label);
                if let Some(window) = window_option {
                  window.show().unwrap();
                }
              }
              WindowEvent::CloseRequested { api, .. } => {
                let window_option = app_handle.get_window(&label);
                if let Some(window) = window_option {
                  window.hide().unwrap();
                  api.prevent_close();
                }
              },
              _ => (),
            }
          },
          _ => (),
        }
      });
    }
    Err(err) => log::error!("Error building the app: {:?}", err),
  }
}

pub fn build_admin_window(app_handle: &AppHandle, local_storage_path: PathBuf) -> Result<Window, tauri::Error> {
  WindowBuilder::new(
    app_handle,
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
    .build()
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
