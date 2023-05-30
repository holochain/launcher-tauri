use crate::{launcher::{state::LauncherState, manager::HolochainId}, file_system::Profile};

#[tauri::command]
pub async fn dump_network_stats(
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  profile: tauri::State<'_, Profile>,
  holochain_id: HolochainId,
) -> Result<String, String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call tauri command 'dump_network_stats' which is not allowed in this window."))
  }

  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  manager
    .get_or_launch_holochain(
      holochain_id,
      profile.inner().clone(),
    ).await?
    .dump_network_stats()
    .await
}
