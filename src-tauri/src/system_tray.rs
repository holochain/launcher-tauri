use std::{collections::HashMap, convert::TryFrom};

use holochain_manager::versions::{
  holochain_conductor_api_latest::InstalledAppInfoStatus, HolochainVersion,
};
use holochain_web_app_manager::installed_web_app_info::{InstalledWebAppInfo, WebUiInfo};
use tauri::{
  window::WindowBuilder, AppHandle, CustomMenuItem, Manager, SystemTrayMenu, SystemTrayMenuItem,
  WindowUrl, Wry,
};

use crate::launcher::{manager::LauncherManager, state::LauncherState};

pub fn handle_system_tray_event(app: &AppHandle<Wry>, event_id: String) {
  match event_id.as_str() {
    "quit" => {
      match LauncherManager::remove_pid_file() {
        Ok(()) => {}
        Err(err) => log::error!("Error removing the pid file app: {:?}", err),
      };

      app.exit(0);
    }
    "show_admin" => {
      let admin_window = app.get_window("admin");

      if let Some(window) = admin_window {
        window.show().unwrap();
        window.set_focus().unwrap();
      } else {
        let r = WindowBuilder::new(app, "admin", WindowUrl::App("index.html".into()))
          .inner_size(1000.0, 700.0)
          .title("Holochain Admin")
          .build();

        log::info!("Creating admin window {:?}", r);
      }
    }
    menu_item_id => {
      let (version, app_id) =
        expand_version_and_app_id(String::from(menu_item_id)).expect("Bad menu item?");

      match app.state::<LauncherState>().get_launcher_manager() {
        Ok(manager) => {
          tauri::async_runtime::block_on(async move {
            if let Err(err) = manager.lock().await.open_app(version, &app_id, app) {
              log::error!("Error opening app: {:?}", err);
            }
          });
        }
        Err(e) => {
          log::error!("Error opening app: {:?}", e);
        }
      };
      ()
    }
  }
}

pub fn builtin_system_tray() -> SystemTrayMenu {
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let show_admin = CustomMenuItem::new("show_admin".to_string(), "Show Admin");

  SystemTrayMenu::new()
    .add_item(show_admin)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit)
}

pub fn update_system_tray(
  app_handle: &AppHandle<Wry>,
  installed_apps_by_version: &HashMap<HolochainVersion, Vec<InstalledWebAppInfo>>,
) -> () {
  let mut menu = builtin_system_tray();

  for (version, installed_apps) in installed_apps_by_version {
    for app in installed_apps {
      if let InstalledAppInfoStatus::Running = app.installed_app_info.status {
        if let WebUiInfo::WebApp { .. } = app.web_ui_info {
          let app_id = app.installed_app_info.installed_app_id;

          menu = menu.add_item(CustomMenuItem::new(
            collapse_version_and_app_id(version.clone(), app_id.clone()),
            app_id.clone(),
          ));
        }
      }
    }
    menu = menu.add_native_item(SystemTrayMenuItem::Separator);
  }

  if let Err(err) = app_handle.tray_handle().set_menu(menu) {
    log::error!("Error setting the system tray: {:?}", err);
  }
}

pub fn collapse_version_and_app_id(holochain_version: HolochainVersion, app_id: String) -> String {
  let version_string: String = holochain_version.into();
  format!("{}:{}", version_string, app_id)
}

pub fn expand_version_and_app_id(
  menu_item_id: String,
) -> Result<(HolochainVersion, String), String> {
  let components: Vec<&str> = menu_item_id.split(":").collect();

  let version = HolochainVersion::try_from(String::from(components[0]))?;

  let app_id = components[1..].join(":");

  Ok((version, app_id))
}
