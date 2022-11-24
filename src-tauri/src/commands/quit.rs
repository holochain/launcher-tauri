
// restarts the Holochain Launcher
#[tauri::command]
pub fn quit(
  window: tauri::Window,
  app_handle: tauri::AppHandle,
) -> Result<(), String> {

  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command. (J)"))
  }

  log::warn!("Quitting the Launcher has been requested, Quitting...");

  app_handle.exit(0);
  Ok(())
}
