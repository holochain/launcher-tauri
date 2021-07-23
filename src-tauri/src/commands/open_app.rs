use std::env;

use crate::uis::port_mapping::PortMapping;

#[tauri::command]
pub fn open_app_ui(app_id: String) -> Result<(), String> {
  let port_mapping = PortMapping::read_port_mapping()?;

  let port = port_mapping
    .get_ui_port_for_app(&app_id)
    .ok_or("App not registered")?;

  let app_url = format!("http://localhost:{}", port);
  env::set_var("BROWSER", "firefox");
  let result = opener::open_browser(app_url.as_str());
  log::info!(
    "Opening app {} at {}, result: {:?}",
    app_id.clone(),
    app_url,
    result
  );

  Ok(())
}
