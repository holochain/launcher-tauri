use crate::{launcher::{state::LauncherState, manager::HolochainId}, file_system::Profile, commands::save_app::{portal_remote_call, fetch_mere_memory}};
use devhub_types::{happ_entry_types::GUIReleaseEntry, Entity, GetEntityInput, FileEntry};
use hdk::prelude::{AnyDhtHash, ActionHash};
use holochain::conductor::api::CellInfo;
use holochain_client::{AgentPubKey, AppWebsocket, AppInfo};
use holochain_types::prelude::{DnaHash, AgentPubKeyB64};
use holochain_web_app_manager::ReleaseInfo;
use mr_bundle::ResourceBytes;


/// Fetching UI assets from the DevHub via remote call through the portal_api
#[tauri::command]
pub async fn fetch_and_update_default_gui(
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  profile: tauri::State<'_, Profile>,
  app_port: u16,
  appstore_app_id: String,
  appstore_pub_key: String,
  host: AgentPubKey, // agent public key of the DevHub host to fetch the webhapp from
  devhub_happ_library_dna_hash: DnaHash, // DNA hash of the DevHub to which the remote call shall be made
  holochain_id: HolochainId,
  app_id: String,
  gui_release_info: Option<ReleaseInfo>,
) -> Result<(), String> {

  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call tauri command 'fetch_and_update_default_gui' which is not allowed in this window."))
  }

  let agent_pub_key = AgentPubKey::from(AgentPubKeyB64::from_b64_str(appstore_pub_key.as_str())
    .map_err(|e| format!("Failed to convert appstorePubKey from Base64 to Vec<u8>: {}", e))?);

  let gui_release_info = match gui_release_info {
    Some(info) => info,
    None => return Err(String::from("gui_release_info is None but must be Some in 'fetch_and_update_default_gui'.")),
  };

  let gui_release_hash = match gui_release_info.resource_locator.clone() {
    Some(locator) => locator.resource_hash,
    None => return Err(String::from("resource_hash of gui_release_info is None but must be Some in 'fetch_and_update_default_gui'.")),
  };

  let gui_release_action_hash = AnyDhtHash::from(gui_release_hash);

  let mut ws = AppWebsocket::connect(format!("ws://localhost:{}", app_port))
      .await
      .map_err(|e| format!("Failed to connect to app websocket: {}", e))?;

  let app_info: AppInfo = ws
      .app_info(appstore_app_id.clone())
      .await
      .map_err(|e| format!("Failed to get appstore AppInfo: {:?}", e))?
      .ok_or(format!("AppInfo is None."))?;

  let cells = app_info.cell_info.get("portal").ok_or(format!("No CellInfo found for portal role"))?;

  let Some(CellInfo::Provisioned(portal_cell)) = cells.get(0) else {
      return Err(format!("No provisioned cell for role portal_api found."));
  };

  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;
  let lair_keystore_manager = manager.get_lair_keystore_manager()?;

  let gui_release_entry_entity: Entity<GUIReleaseEntry> = portal_remote_call(
    &mut ws,
    lair_keystore_manager,
    &agent_pub_key,
    host.clone(),
    portal_cell,
    devhub_happ_library_dna_hash.clone(),
    String::from("happ_library"),
    String::from("get_gui_release"),
    GetEntityInput {
      id: ActionHash::try_from(gui_release_action_hash)
        .map_err(|err| format!("Failed to convert AnDhtHash to ActionHash: {:?}", err))?,
    }
  ).await?;

  let web_asset_file: Entity<FileEntry> = portal_remote_call(
    &mut ws,
    lair_keystore_manager,
    &agent_pub_key,
    host.clone(),
    portal_cell,
    devhub_happ_library_dna_hash.clone(),
    String::from("happ_library"),
    String::from("get_webasset_file"),
    GetEntityInput {
      id: gui_release_entry_entity.content.web_asset_id,
    }
  ).await?;

  let ui_bytes = fetch_mere_memory(
    &mut ws,
    lair_keystore_manager,
    &agent_pub_key,
    host.clone(),
    portal_cell,
    "web_assets",
    devhub_happ_library_dna_hash.clone(),
    web_asset_file.content.mere_memory_addr
  ).await?;

  let default_ui_name = String::from("default");

  log::info!("Installing: New UI for app '{}'", &app_id);

  manager
    .get_or_launch_holochain(
      holochain_id,
      profile.inner().clone(),
    ).await?
    .update_app_ui(
      app_id.clone(),
      ResourceBytes::from(ui_bytes),
      &default_ui_name,
      Some(gui_release_info),
    )?;

  log::info!("Installed new UI for app '{}'", app_id);

  manager.on_apps_changed().await?;

  Ok(())
}





#[tauri::command]
pub async fn update_default_ui(
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  profile: tauri::State<'_, Profile>,
  holochain_id: HolochainId,
  app_id: String,
  ui_zip_bytes: Vec<u8>,
  gui_release_info: Option<ReleaseInfo>,
) -> Result<(), String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command. (Q)"))
  }

  let default_ui_name = String::from("default");

  log::info!("Installing: New UI for app '{}'", &app_id);
  if gui_release_info.is_none() {
    log::warn!("WARNING: No GUI release hash passed to update_ui command. Automatically checking for updates will not work for this UI.");
  }

  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  manager
    .get_or_launch_holochain(
      holochain_id,
      profile.inner().clone(),
    ).await?
    .update_app_ui(
      app_id.clone(),
      ResourceBytes::from(ui_zip_bytes),
      &default_ui_name,
      gui_release_info,
    )?;

  log::info!("Installed new UI for app '{}'", app_id);

  manager.on_apps_changed().await?;

  Ok(())
}
