use crate::{commands::open_app::{open_url, report_issue}, setup::logs};
use tauri::{CustomMenuItem, Manager, Menu, Submenu, Window, Wry};

pub fn build_menu() -> Menu {
  let factory_reset = CustomMenuItem::new("factory_reset".to_string(), "Factory Reset");
  let open_logs = CustomMenuItem::new("open_logs".to_string(), "Open Logs");
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");

  let settings_submenu = Submenu::new(
    "Settings",
    Menu::new()
      .add_item(factory_reset)
      .add_item(open_logs)
      .add_item(quit),
  );
  let about = CustomMenuItem::new("about".to_string(), "About");
  let report_issue = CustomMenuItem::new("report_issue".to_string(), "Report Issue");
  let help_submenu = Submenu::new("Help", Menu::new().add_item(about).add_item(report_issue));
  Menu::new()
    .add_submenu(settings_submenu)
    .add_submenu(help_submenu)
}

pub fn handle_menu_event(event_id: &str, window: &Window<Wry>) {
  match event_id {
    "factory_reset" => window.emit("request-factory-reset", ()).unwrap(),
    "about" => window.emit("about", ()).unwrap(),
    "quit" => {
      window.app_handle().exit(0);
    }
    "report_issue" => report_issue(),
    "open_logs" => {
      logs::open_logs_folder();
    }
    _ => {}
  }
}
