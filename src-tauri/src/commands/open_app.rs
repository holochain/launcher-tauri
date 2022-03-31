use holochain_manager::versions::HolochainVersion;
use std::io;

use crate::launcher::state::LauncherState;

#[tauri::command]
pub async fn open_app_ui(
  state: tauri::State<'_, LauncherState>,
  app_handle: tauri::AppHandle,
  holochain_version: HolochainVersion,
  app_id: String,
) -> Result<(), String> {
  let mut manager = state.get_launcher_manager()?.lock().await;

  manager
    .open_app(holochain_version, &app_id)
    .map_err(|err| format!("Error opening app: {}", err))?;

  log::info!("Opening app {}", app_id.clone(),);

  Ok(())
}

#[tauri::command]
pub fn report_issue() -> () {
  open_url("https://github.com/holochain/launcher/issues/new?assignees=&labels=bug&template=bug_report.md&title=".into()).unwrap();
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
