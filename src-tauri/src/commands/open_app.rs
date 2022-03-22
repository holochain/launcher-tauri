use std::io;

use crate::state::LauncherState;

#[tauri::command]
pub async fn open_app_ui(
  state: tauri::State<'_, LauncherState>,
  app_handle: tauri::AppHandle,
  app_id: String,
) -> Result<(), String> {
  let mut manager = state.get_launcher_manager()?.lock().await;

  manager
    .get_holochain_manager()?
    .ui_manager
    .open_app(&app_id, &app_handle)
    .map_err(|err| format!("Error opening app: {}", err))?;

  log::info!("Opening app {}", app_id.clone(),);

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