use std::{process::Child, sync::Arc};

use crate::state::HolochainLauncherState;

#[tauri::command]
pub fn deactivate_app_ui(
  state: tauri::State<HolochainLauncherState>,
  app_id: String,
) -> Result<(), String> {
  let arc = Arc::clone(&state.caddy_processes);
  let mut caddy_processes = arc.lock().unwrap();

  let ui_process: &mut Child = (*caddy_processes)
    .get_mut(&app_id)
    .ok_or(String::from("App UI is not running"))?;

  ui_process
    .kill()
    .map_err(|err| format!("Could not kill caddy UI process: {:?}", err))?;
  caddy_processes.remove(&app_id);

  Ok(())
}
