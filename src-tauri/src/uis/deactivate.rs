/* use std::sync::Arc;
use tauri::api::process::CommandChild; */

use crate::state::HolochainLauncherState;

#[tauri::command]
pub fn deactivate_app_ui(
  _state: tauri::State<HolochainLauncherState>,
  _app_id: String,
) -> Result<(), String> {
  /* let arc = Arc::clone(&state.caddy_processes);
  let caddy_processes = arc.lock().unwrap();

  let ui_process: CommandChild = *(caddy_processes
    .get(&app_id)
    .ok_or(String::from("App UI is not running"))?);

  ui_process
    .kill()
    .map_err(|err| format!("Could not kill caddy UI process: {:?}", err))?;
  caddy_processes.remove(&app_id); */

  Ok(())
}
