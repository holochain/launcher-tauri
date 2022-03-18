use std::io;

use crate::{state::LauncherState, uis::port_mapping::PortMapping};

#[tauri::command]
pub fn open_app_ui(
  state: tauri::State<'_, LauncherState>,
  app_handle: tauri::AppHandle,
  app_id: String,
) -> Result<(), String> {
  let manager = state.get_holochain_manager()?;

  manager
    .ui_manager
    .open_app(app_id, &app_handle)
    .map_err(|err| format!("Error opening app: {}", err))?;

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
