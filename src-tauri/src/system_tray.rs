use std::collections::HashMap;

use holochain_manager::versions::{
  holochain_conductor_api_latest::AppInfoStatus, HolochainVersion,
};
use holochain_web_app_manager::installed_web_app_info::{InstalledWebAppInfo, WebUiInfo};
use tauri::{
  window::WindowBuilder, AppHandle, CustomMenuItem, Manager, SystemTrayMenu, SystemTrayMenuItem,
  WindowUrl, Wry,
};

use crate::launcher::{state::LauncherState, manager::HolochainId};

pub fn handle_system_tray_event(app: &AppHandle<Wry>, event_id: String) {
  match event_id.as_str() {
    "quit" => {
      app.exit(0);
    },
    "restart" => {
      app.app_handle().restart();
    },
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
        expand_holochain_and_app_id(String::from(menu_item_id)).expect("Bad menu item?");

      let launcher_state = app.state::<LauncherState>();
      tauri::async_runtime::block_on(async move {
        let mut mutex = (*launcher_state).lock().await;

        match mutex.get_running() {
          Ok(manager) => {
            if let Err(err) = manager.open_app(version, &app_id) {
              log::error!("Error opening app: {:?}", err);
            }
          }
          Err(e) => {
            log::error!("Error opening app: {:?}", e);
          }
        };
      });

      ()
    }
  }
}

pub fn initial_system_tray() -> SystemTrayMenu {
  let mut menu = SystemTrayMenu::new();

  for item in builtin_system_tray() {
    menu = menu.add_item(item);
  }
  menu
}

pub fn builtin_system_tray() -> Vec<CustomMenuItem> {
  vec![
    CustomMenuItem::new("show_admin".to_string(), "Show Admin"),
    CustomMenuItem::new("restart".to_string(), "Restart"),
    CustomMenuItem::new("quit".to_string(), "Quit"),
  ]
}

pub struct AllInstalledApps {
  pub by_version: HashMap<HolochainVersion, Vec<InstalledWebAppInfo>>,
  pub custom_binary: Option<Vec<InstalledWebAppInfo>>,
}

pub fn update_system_tray(
  app_handle: &AppHandle<Wry>,
  all_installed_apps: &AllInstalledApps,
) -> () {
  let mut menu = SystemTrayMenu::new();

  for (version, installed_apps) in &all_installed_apps.by_version {
    for app in installed_apps {
      if let AppInfoStatus::Running = app.installed_app_info.status {
        if let WebUiInfo::WebApp { .. } = app.web_ui_info {
          let app_id = app.installed_app_info.installed_app_id.clone();

          menu = menu.add_item(CustomMenuItem::new(
            collapse_holochain_and_app_id(
              HolochainId::HolochainVersion(version.clone()),
              app_id.clone(),
            ),
            app_id.clone(),
          ));
        }
      }
    }
    menu = menu.add_native_item(SystemTrayMenuItem::Separator);
  }

  if let Some(custom_binary_apps) = &all_installed_apps.custom_binary {
    for app in custom_binary_apps {
      if let AppInfoStatus::Running = app.installed_app_info.status {
        if let WebUiInfo::WebApp { .. } = app.web_ui_info {
          let app_id = app.installed_app_info.installed_app_id.clone();

          menu = menu.add_item(CustomMenuItem::new(
            collapse_holochain_and_app_id(HolochainId::CustomBinary, app_id.clone()),
            app_id.clone(),
          ));
        }
      }
    }
  }

  for item in builtin_system_tray() {
    menu = menu.add_item(item);
  }
  if let Err(err) = app_handle.tray_handle().set_menu(menu) {
    log::error!("Error setting the system tray: {:?}", err);
  }
}

pub fn collapse_holochain_and_app_id(holochain_id: HolochainId, app_id: String) -> String {
  match holochain_id {
    HolochainId::HolochainVersion(holochain_version) => {
      let version_string: String = holochain_version.into();
      format!("{}:{}", version_string, app_id)
    }
    HolochainId::CustomBinary => format!("custom_binary:{}", app_id),
  }
}

pub fn expand_holochain_and_app_id(menu_item_id: String) -> Result<(HolochainId, String), String> {
  let components: Vec<&str> = menu_item_id.split(":").collect();

  let app_id = components[1..].join(":");

  match components[0] {
    "custom" => Ok((HolochainId::CustomBinary, app_id)),
    version_str => {
      let version = version_str
        .parse::<HolochainVersion>()
        .or(Err(String::from("Invalid version")))?;

      Ok((HolochainId::HolochainVersion(version), app_id))
    }
  }
}
