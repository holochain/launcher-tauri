use std::env;

use opener::OpenError;

use crate::uis::port_mapping::PortMapping;

#[tauri::command]
pub fn open_app_ui(app_id: String) -> Result<(), String> {
  let port_mapping = PortMapping::read_port_mapping()?;

  let port = port_mapping
    .get_ui_port_for_app(&app_id)
    .ok_or("App not registered")?;

  let app_url = format!("http://localhost:{}", port);
  let result = open_url(app_url.as_str());
  log::info!(
    "Opening app {} at {}, result: {:?}",
    app_id.clone(),
    app_url,
    result
  );

  Ok(())
}

pub fn open_url(url: &str) -> Result<(), OpenError> {
  env::set_var("BROWSER", "firefox");

  if let Err(_) = opener::open(url) {
    env::remove_var("BROWSER");
    return opener::open(url);
  }
  Ok(())
}
