use tauri::{
  window::WindowBuilder, AppHandle, CustomMenuItem, Manager, SystemTrayMenu, SystemTrayMenuItem,
  WindowUrl, Wry,
};

use crate::state::LauncherState;

pub fn handle_system_tray_event(app: &AppHandle<Wry>, event_id: String) {
  match event_id.as_str() {
    "quit" => {
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
      let launcher_state: LauncherState = *app.state();

      match launcher_state.get_holochain_manager() {
        Ok(manager) => {
          manager.ui_manager.open_app(String::from(app_id), app);
        }
        Err(e) => log::error!("Error opening app: {:?}", e),
      }
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
  running_apps: Vec<String>,
) -> () {
  let mut menu = builtin_system_tray();

  for app in running_apps {
    menu.add_item(CustomMenuItem::new(app.clone(), app));
  }

  app_handle.tray_handle().set_menu(menu);

  Ok(())
}
