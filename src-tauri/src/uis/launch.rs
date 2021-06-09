use open;

use super::port_mapping::PortMapping;

#[tauri::command]
pub fn launch_app_ui(app_id: String) -> Result<String, String> {
  let port_mapping = PortMapping::read_port_mapping()?;

  let port = port_mapping
    .get_ui_port_for_app(&app_id)
    .ok_or("App not registered")?;

  let app_url = format!("http://localhost:{}", port);
  let result = open::that(app_url).or(Err(String::from("Cannot open app ui")))?;

  Ok(format!("UI launched: {:?}", result))
}
