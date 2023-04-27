use crate::{launcher::{state::LauncherState, manager::HolochainId}, file_system::Profile, BootstrapServerUrl, SignalingServerUrl};
use mr_bundle::ResourceBytes;

#[tauri::command]
pub async fn update_default_ui(
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  profile: tauri::State<'_, Profile>,
  bootstrap_server_url: tauri::State<'_, BootstrapServerUrl>,
  signaling_server_url: tauri::State<'_, SignalingServerUrl>,
  holochain_id: HolochainId,
  app_id: String,
  ui_zip_bytes: Vec<u8>,
  gui_release_hash: Option<String>,
) -> Result<(), String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command. (Q)"))
  }

  let default_ui_name = String::from("default");

  log::info!("Installing: New UI for app '{}'", &app_id);
  if gui_release_hash == None {
    log::warn!("WARNING: No GUI release hash passed to update_ui command. Automatically checking for updates will not work for this UI.");
  }

  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  manager
    .get_or_launch_holochain(
      holochain_id,
      profile.inner().clone(),
      bootstrap_server_url.inner().to_owned(),
      signaling_server_url.inner().to_owned()
    ).await?
    .update_app_ui(
      app_id.clone(),
      ResourceBytes::from(ui_zip_bytes),
      &default_ui_name,
      gui_release_hash,
    )?;

  log::info!("Installed new UI for app '{}'", app_id);

  manager.on_apps_changed().await?;

  Ok(())
}
