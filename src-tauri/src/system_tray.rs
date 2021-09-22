use tauri::{AppHandle, CustomMenuItem, Manager, SystemTray, SystemTrayMenu, SystemTrayMenuItem, WindowBuilder, WindowUrl, Wry};


pub fn build_system_tray() -> SystemTray {
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let show_admin = CustomMenuItem::new("show_admin".to_string(), "Show Admin");

  let sys_tray_menu = SystemTrayMenu::new()
    .add_item(show_admin)
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(quit);

  SystemTray::new().with_menu(sys_tray_menu)
}

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
    _ => {}
  }
}
