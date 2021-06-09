use open;

use super::port_mapping::PortMapping;

#[tauri::command]
pub fn open_app_ui(app_id: String) -> Result<String, String> {
  let port_mapping = PortMapping::read_port_mapping()?;

  let port = port_mapping
    .get_ui_port_for_app(&app_id)
    .ok_or("App not registered")?;

  let app_url = format!("http://localhost:{}", port);
  let result = open::with(app_url.as_str(), "firefox")
    .map_err(|err| format!("Cannot open app ui: {:?}", err))?;

  Ok(format!("UI launched: {:?}", result))
}
