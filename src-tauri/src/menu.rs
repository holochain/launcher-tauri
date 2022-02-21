use tauri::{CustomMenuItem, Menu, Submenu, Window, Wry};

use crate::{commands::open_app::open_url, setup::logs};

pub fn build_menu() -> Menu {
  let factory_reset = CustomMenuItem::new("factory-reset".to_string(), "Factory Reset");
  let open_logs = CustomMenuItem::new("open_logs".to_string(), "Open Logs");
  let install_plugin = CustomMenuItem::new("install-plugin".to_string(), "Install Plugin");

  let settings_submenu = Submenu::new(
    "Settings",
    Menu::new().add_item(factory_reset).add_item(open_logs).add_item(install_plugin),
  );
  let about = CustomMenuItem::new("about".to_string(), "About");
  let report_issue = CustomMenuItem::new("report-issue".to_string(), "Report Issue");
  let help_submenu = Submenu::new("Help", Menu::new().add_item(about).add_item(report_issue));
  Menu::new()
    .add_submenu(settings_submenu)
    .add_submenu(help_submenu)
}

pub fn handle_menu_event(event_id: &str, window: &Window<Wry>) {
  match event_id {
    "factory-reset" => window.emit("request-factory-reset", ()).unwrap(),
    "about" => window.emit("about", ()).unwrap(),
    "report-issue" => open_url("https://github.com/holochain/launcher/issues/new?assignees=&labels=bug&template=bug_report.md&title=".into()).unwrap(),
    "open_logs" => {
      logs::open_logs_folder();
    },
    "install-plugin" => window.emit("request-plugin-install", ()).unwrap(),
    _ => {}
  }
}
