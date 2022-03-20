use tauri::{
  window::WindowBuilder, AppHandle, CustomMenuItem, Manager, SystemTrayMenu, SystemTrayMenuItem,
  WindowUrl, Wry,
};

use crate::{managers::launcher::LauncherManager, state::LauncherState};

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
    app_id => {
      match app.state::<LauncherState>().get_launcher_manager() {
        Ok(manager) => {
          tauri::async_runtime::block_on(async move {
            if let Ok(holochain_manager) = manager.lock().await.get_holochain_manager() {
              if let Err(err) = holochain_manager
                .ui_manager
                .open_app(&String::from(app_id), app)
              {
                log::error!("Error opening app: {:?}", err);
              }
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

pub fn update_system_tray(app_handle: &AppHandle<Wry>, running_apps: &Vec<String>) -> () {
  let mut menu = builtin_system_tray();

  for app in running_apps {
    menu = menu.add_item(CustomMenuItem::new(app.clone(), app.clone()));
  }

  if let Err(err) = app_handle.tray_handle().set_menu(menu) {
    log::error!("Error setting the system tray: {:?}", err);
  }
}
