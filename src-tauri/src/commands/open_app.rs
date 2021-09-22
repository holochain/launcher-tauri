use std::io;

use crate::uis::port_mapping::PortMapping;

#[tauri::command]
pub fn open_app_ui(app_id: String) -> Result<(), String> {
  let port_mapping = PortMapping::read_port_mapping()?;

  let port = port_mapping
    .get_ui_port_for_app(&app_id)
    .ok_or("App not registered")?;

  let app_url = format!("http://localhost:{}", port);
  let result = open_url(app_url.clone());
  log::info!(
    "Opening app {} at {}, result: {:?}",
    app_id.clone(),
    app_url,
    result
  );

  Ok(())
}

pub fn open_url(url: String) -> io::Result<()> {
  tauri::async_runtime::spawn(async move {
    if let Err(_) = open::with(url.clone().as_str(), "firefox") {
      return open::that(url.clone().as_str());
    }
    Ok(())
  });

  Ok(())
}
