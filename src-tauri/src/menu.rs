use crate::{setup::logs, quit};
use tauri::{CustomMenuItem, Manager, Menu, Submenu, Window, Wry};
use crate::file_system::Profile;

pub fn build_menu() -> Menu {
  let factory_reset = CustomMenuItem::new("factory_reset".to_string(), "Factory Reset");
  let open_logs = CustomMenuItem::new("open_logs".to_string(), "Open Logs");
  // let config = CustomMenuItem::new("config".to_string(), "Configuration");
  let language_settings = CustomMenuItem::new("language_settings".to_string(), "Language");
  let network_stats = CustomMenuItem::new("network_stats".to_string(), "Network Statistics");
  let restart = CustomMenuItem::new("restart".to_string(), "Restart");
  let quit = CustomMenuItem::new("quit".to_string(), "Quit");
  let version_info = CustomMenuItem::new("about".to_string(), "Version Info");
  // let report_issue = CustomMenuItem::new("report_issue".to_string(), "Report Issue");

  let menu_submenu = Submenu::new(
    "Menu",
    Menu::new()
      .add_item(version_info.clone())
      .add_item(open_logs.clone())
      .add_item(restart.clone())
      .add_item(quit.clone()),
  );
  let settings_submenu = Submenu::new(
    "Settings",
    Menu::new()
      .add_item(language_settings.clone())
      .add_item(network_stats.clone())
      // .add_item(config.clone())
      .add_item(factory_reset.clone())
  );



  // special menu for macOS
  if cfg!(target_os = "macos") {
    let launcher_menu_submenu = Submenu::new(
      "Launcher",
      Menu::new()
        .add_item(version_info)
        .add_item(open_logs)
        .add_item(restart)
        .add_item(quit),
    );

    let settings_submenu_macos = Submenu::new(
      "Settings",
      Menu::new()
        .add_item(language_settings.clone())
        .add_item(network_stats.clone())
        // .add_item(config)
        .add_item(factory_reset)
    );

    return Menu::os_default("Holochain Launcher")
      .add_submenu(launcher_menu_submenu)
      .add_submenu(settings_submenu_macos)
  }

  Menu::new()
    .add_submenu(menu_submenu)
    .add_submenu(settings_submenu)
}

pub fn handle_menu_event(event_id: &str, window: &Window<Wry>) {
  let app_handle = window.app_handle();
  let profile = app_handle.state::<Profile>();
  match event_id {
    "factory_reset" => window.emit("request-factory-reset", ()).unwrap(),
    // "config" => window.emit("open-config", ()).unwrap(),
    "language_settings" => window.emit("open-language-settings", ()).unwrap(),
    "network_stats" => window.emit("open-network-stats", ()).unwrap(),
    "about" => window.emit("about", ()).unwrap(),
    "restart" => window.emit("request-restart", ()).unwrap(),
    "quit" => {
      quit(window.clone(), window.app_handle()).unwrap();
    }
    // "report_issue" => report_issue(),
    "open_logs" => {
      logs::open_logs_folder(profile.inner().clone());
    }
    _ => {}
  }
}
