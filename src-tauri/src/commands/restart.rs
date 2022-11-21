


// restarts the Holochain Launcher
#[tauri::command]
pub fn restart(
  window: tauri::Window,
  app_handle: tauri::AppHandle,
) -> Result<(), String> {

  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command. (J)"))
  }

  log::warn!("A Restart of the Launcher has been requested, restarting...");

  app_handle.restart();
  Ok(())
}
