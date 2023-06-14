use std::{env::temp_dir, fs, path::PathBuf, time::SystemTime};


use devhub_types::{DevHubResponse, Entity, web_asset_entry_types::FilePackage};
use holochain_types::{prelude::{DnaHash, AgentPubKeyB64, EntryHashB64}, web_app::WebAppBundle};
use lair_keystore_manager::LairKeystoreManager;
use holochain_manager::versions::holochain_conductor_api_latest::CellInfo;
use holochain_state::nonce::fresh_nonce;
use holochain_client::{AppInfo, AppWebsocket, AgentPubKey};
use hdk::prelude::{
  EntryHash, ExternIO, FunctionName, Serialize, Timestamp, ZomeCallUnsigned, ZomeName, Deserialize
};

use crate::{launcher::{state::LauncherState, manager::HolochainId}, file_system::Profile};


#[tauri::command]
pub fn save_app(
  window: tauri::Window,
  app_bundle_bytes: Vec<u8>,
) -> Result<PathBuf, String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call tauri command 'save_app' which is not allowed in this window."))
  }

  let now = SystemTime::now();

  let path = temp_dir().join(format!("app_to_install{:?}.webhapp", now));

  fs::write(path.clone(), app_bundle_bytes)
    .map_err(|err| format!("Failed to write app bundle: {}", err))?;

  Ok(path)
}



/// Fetches an app from a DevHub Host and stores it to a temp path while the UI
/// is asking for membrane proofs ()
#[tauri::command]
pub async fn fetch_and_save_app(
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  profile: tauri::State<'_, Profile>,
  holochain_id: HolochainId,
  appstore_app_id: String,
  app_title: String,
  host: AgentPubKey,
  devhub_happ_library_dna_hash: DnaHash, // DNA hash of the DevHub to which the remote call shall be made
  appstore_pub_key: String,
  happ_release_hash: String,
  gui_release_hash: String,
) -> Result<PathBuf, String> {

  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call tauri command 'fetch_and_save_app' which is not allowed in this window."))
  }

  let appstore_pub_key = AgentPubKey::from(AgentPubKeyB64::from_b64_str(appstore_pub_key.as_str())
    .map_err(|e| String::from("Failed to convert appstorePubKey from Base64 to Vec<u8>"))?);
  let happ_release_entry_hash = EntryHash::from(EntryHashB64::from_b64_str(happ_release_hash.as_str())
    .map_err(|e| String::from("Failed to convert happReleaseHash from Base64 to Vec<u8>"))?);
  let gui_release_entry_hash = EntryHash::from(EntryHashB64::from_b64_str(gui_release_hash.as_str())
    .map_err(|e| String::from("Failed to convert guiReleaseHash from Base64 to Vec<u8>"))?);

  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  let bytes = fetch_web_happ(
      manager.get_or_launch_holochain(
        holochain_id,
        profile.inner().clone(),
      ).await?.app_interface_port(),
      &appstore_app_id,
      &appstore_pub_key,
      manager.get_lair_keystore_manager()?,
      host,
      devhub_happ_library_dna_hash,
      app_title,
      happ_release_entry_hash,
      gui_release_entry_hash,
  )
  .await?;

  let _web_app_bundle = WebAppBundle::decode(&bytes)
    .or(Err(String::from("Failed to read Web hApp bundle file")))?;

  let now = SystemTime::now();

  let path = temp_dir().join(format!("app_to_install{:?}.webhapp", now));

  fs::write(path.clone(), bytes)
    .map_err(|err| format!("Failed to write app bundle: {}", err))?;

  Ok(path)
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteCallDetails<Z,F,I>
where
    Z: Into<ZomeName>,
    F: Into<FunctionName>,
    I: Serialize + core::fmt::Debug,
{
    pub dna: DnaHash,
    pub zome: Z,
    pub function: F,
    pub payload: I,
}


// #[derive(Debug, Deserialize)]
// pub struct CustomRemoteCallInput {
//     host: AgentPubKey,
//     call: RemoteCallInput,
// }

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchWebHappRemoteCallInput {
  host: AgentPubKey,
  call: RemoteCallDetails<String, String, GetWebHappPackageInput>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetWebHappPackageInput {
    pub name: String,
    pub happ_release_id: EntryHash,
    pub gui_release_id: EntryHash,
}

/// Fetching a webhapp from the DevHub via remote call through the portal_api
async fn fetch_web_happ(
  app_port: u16,
  appstore_app_id: &String,
  agent_pub_key: &AgentPubKey,
  lair_keystore_manager: &Box<dyn LairKeystoreManager>,
  host: AgentPubKey, // agent public key of the DevHub host to fetch the webhapp from
  devhub_happ_library_dna_hash: DnaHash, // DNA hash of the DevHub to which the remote call shall be made
  name: String, // name of the webhapp to use in the WebHappManifest
  happ_release_entry_hash: EntryHash,
  gui_release_entry_hash: EntryHash,
) -> Result<Vec<u8>, String> {
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

  let (nonce, expires_at) = fresh_nonce(Timestamp::now())
      .map_err(|e| format!("Failed to create fresh Nonce: {:?}", e))?;

  let payload = GetWebHappPackageInput {
    name,
    happ_release_id: happ_release_entry_hash,
    gui_release_id: gui_release_entry_hash,
  };

  let input = FetchWebHappRemoteCallInput {
    host,
    call: RemoteCallDetails {
      dna: devhub_happ_library_dna_hash,
      zome: String::from("happ_library"),
      function: String::from("get_webhapp_package"),
      payload,
    }
  };

  let zome_call_unsigned = ZomeCallUnsigned {
      provenance: agent_pub_key.clone(),
      cell_id: portal_cell.cell_id.clone(),
      zome_name: ZomeName::from("portal_api"),
      fn_name: FunctionName::from("custom_remote_call"),
      payload: ExternIO::encode(input)?,
      cap_secret: None,
      expires_at,
      nonce,
  };


  let signed_zome_call = lair_keystore_manager
      .sign_zome_call(zome_call_unsigned)
      .await
      .map_err(|e| format!("Failed to sign zome call: {}", e))?;

  let result = ws
      .call_zome(signed_zome_call)
      .await
      .map_err(|e| format!("Zome call failed: {:?}", e))?;


  let response: DevHubResponse<DevHubResponse<Vec<u8>>> = result.decode()
    .map_err(|e| format!("Error decoding the webhapp package: {}", e))?;

  let inner_response = match response {
    DevHubResponse::Success(pack) => pack.payload,
    DevHubResponse::Failure(error) => {
      println!("Errorpayload: {:?}", error.payload);
      return Err(format!("Received ErrorPayload: {:?}", error.payload));
    },
  };

  let bytes = inner_response
      .as_result()
      .map_err(|e| format!("Failed to get content from DevHubResponse: {}", e))?;

  Ok(bytes)
}




#[derive(Debug, Serialize, Deserialize)]
pub struct FetchWebAssetRemoteCallInput {
  host: AgentPubKey,
  call: RemoteCallDetails<String, String, GetWebAssetInput>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetWebAssetInput {
    pub id: EntryHash,
}


/// Fetching UI assets from the DevHub via remote call through the portal_api
#[tauri::command]
pub async fn fetch_gui(
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  app_port: u16,
  appstore_app_id: String,
  appstore_pub_key: String,
  host: AgentPubKey, // agent public key of the DevHub host to fetch the webhapp from
  devhub_happ_library_dna_hash: DnaHash, // DNA hash of the DevHub to which the remote call shall be made
  gui_release_hash: String,
) -> Result<Vec<u8>, String> {

  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call tauri command 'fetch_gui' which is not allowed in this window."))
  }

  let agent_pub_key = AgentPubKey::from(AgentPubKeyB64::from_b64_str(appstore_pub_key.as_str())
    .map_err(|e| String::from("Failed to convert appstorePubKey from Base64 to Vec<u8>"))?);

  let gui_release_entry_hash = EntryHash::from(EntryHashB64::from_b64_str(gui_release_hash.as_str())
    .map_err(|e| String::from("Failed to convert guiReleaseHash from Base64 to Vec<u8>"))?);


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

  let (nonce, expires_at) = fresh_nonce(Timestamp::now())
      .map_err(|e| format!("Failed to create fresh Nonce: {:?}", e))?;


  let payload = GetWebAssetInput {
    id: gui_release_entry_hash,
  };

  let input = FetchWebAssetRemoteCallInput {
    host,
    call: RemoteCallDetails {
      dna: devhub_happ_library_dna_hash,
      zome: String::from("happ_library"),
      function: String::from("get_webasset"),
      payload,
    }
  };

  let zome_call_unsigned = ZomeCallUnsigned {
      provenance: agent_pub_key.clone(),
      cell_id: portal_cell.cell_id.clone(),
      zome_name: ZomeName::from("portal_api"),
      fn_name: FunctionName::from("custom_remote_call"),
      payload: ExternIO::encode(input)?,
      cap_secret: None,
      expires_at,
      nonce,
  };

  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;
  let lair_keystore_manager = manager.get_lair_keystore_manager()?;

  let signed_zome_call = lair_keystore_manager
      .sign_zome_call(zome_call_unsigned)
      .await
      .map_err(|e| format!("Failed to sign zome call: {}", e))?;

  let result = ws
      .call_zome(signed_zome_call)
      .await
      .map_err(|e| format!("Zome call failed: {:?}", e))?;


  let response: DevHubResponse<DevHubResponse<Entity<FilePackage>>> = result.decode()
    .map_err(|e| format!("Error decoding the webhapp package: {}", e))?;

  let inner_response = match response {
    DevHubResponse::Success(pack) => pack.payload,
    DevHubResponse::Failure(error) => {
      println!("Errorpayload: {:?}", error.payload);
      return Err(format!("Received ErrorPayload: {:?}", error.payload));
    },
  };

  let entity = inner_response
      .as_result()
      .map_err(|e| format!("Failed to get content from DevHubResponse: {}", e))?;

  match entity.content.bytes {
    Some(bytes) => Ok(bytes),
    None => Err(String::from("FilePackage contained no bytes.")),
  }
}
