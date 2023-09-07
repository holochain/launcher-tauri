use std::{collections::BTreeMap, env::temp_dir, fs, path::PathBuf, time::SystemTime};

use devhub_types::{
  encode_bundle, happ_entry_types::GUIReleaseEntry, happ_entry_types::HappManifest, DevHubResponse,
  DnaVersionEntry, Entity, FileEntry, GetEntityInput, HappReleaseEntry,
};
use hdk::prelude::{
  ActionHash, Deserialize, EntryHash, ExternIO, FunctionName, HumanTimestamp, Serialize, Timestamp,
  ZomeCallUnsigned, ZomeName,
};
use holochain::conductor::api::ProvisionedCell;
use holochain_client::{AgentPubKey, AppInfo, AppWebsocket};
use holochain_manager::versions::holochain_conductor_api_latest::CellInfo;
use holochain_state::nonce::fresh_nonce;
use holochain_types::prelude::{ActionHashB64, AgentPubKeyB64, DnaHash};
use lair_keystore_manager::LairKeystoreManager;
use mere_memory_types::{MemoryBlockEntry, MemoryEntry};
// use mere_memory_types::MemoryEntry;
use serde::de::DeserializeOwned;

use crate::{
  file_system::Profile,
  launcher::{manager::HolochainId, state::LauncherState},
};

#[tauri::command]
pub fn save_app(window: tauri::Window, app_bundle_bytes: Vec<u8>) -> Result<PathBuf, String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call tauri command 'save_app' which is not allowed in this window."));
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
  happ_release_hash: String, // ActionHash of the HappReleaseEntry Record
) -> Result<PathBuf, String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call tauri command 'fetch_and_save_app' which is not allowed in this window."));
  }

  let appstore_pub_key = AgentPubKey::from(
    AgentPubKeyB64::from_b64_str(appstore_pub_key.as_str()).map_err(|e| {
      format!(
        "Failed to convert appstorePubKey from Base64 to Vec<u8>: {}",
        e
      )
    })?,
  );
  let happ_release_action_hash = ActionHash::from(
    ActionHashB64::from_b64_str(happ_release_hash.as_str()).map_err(|e| {
      format!(
        "Failed to convert happReleaseHash from Base64 to Vec<u8>: {}",
        e
      )
    })?,
  );

  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  let app_port = manager
    .get_or_launch_holochain(holochain_id, profile.inner().clone())
    .await?
    .app_interface_port();

  let mut ws = AppWebsocket::connect(format!("ws://localhost:{}", app_port))
    .await
    .map_err(|e| format!("Failed to connect to app websocket: {}", e))?;

  let app_info: AppInfo = ws
    .app_info(appstore_app_id.clone())
    .await
    .map_err(|e| format!("Failed to get appstore AppInfo: {:?}", e))?
    .ok_or(format!("AppInfo is None."))?;

  let cells = app_info
    .cell_info
    .get("portal")
    .ok_or(format!("No CellInfo found for portal role"))?;

  let Some(CellInfo::Provisioned(portal_cell)) = cells.get(0) else {
    return Err(format!("No provisioned cell for role portal_api found."));
  };

  // fetch HappReleaseEntry to check whether it's a happ or webhapp
  let happ_release_entry_entity: Entity<HappReleaseEntry> = portal_remote_call(
    &mut ws,
    manager.get_lair_keystore_manager()?,
    &appstore_pub_key,
    host.clone(),
    portal_cell,
    devhub_happ_library_dna_hash.clone(),
    String::from("happ_library"),
    String::from("get_happ_release"),
    GetEntityInput {
      id: happ_release_action_hash,
    },
  )
  .await?;

  let now = SystemTime::now();

  match happ_release_entry_entity.content.official_gui.clone() {
    Some(_gui_hash) => {
      let bytes = fetch_and_assemble_web_happ(
        &mut ws,
        manager.get_lair_keystore_manager()?,
        &appstore_pub_key,
        host,
        portal_cell,
        devhub_happ_library_dna_hash,
        happ_release_entry_entity.content,
        app_title,
      )
      .await?;

      let path = temp_dir().join(format!("app_to_install{:?}.webhapp", now));

      fs::write(path.clone(), bytes)
        .map_err(|err| format!("Failed to write webhapp bundle: {}", err))?;

      Ok(path)
    }
    None => {
      let bytes = fetch_and_assemble_happ(
        &mut ws,
        manager.get_lair_keystore_manager()?,
        &appstore_pub_key,
        host,
        portal_cell,
        devhub_happ_library_dna_hash,
        happ_release_entry_entity.content,
      )
      .await?;

      let path = temp_dir().join(format!("app_to_install{:?}.happ", now));

      fs::write(path.clone(), bytes)
        .map_err(|err| format!("Failed to write happ bundle: {}", err))?;

      Ok(path)
    }
  }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RemoteCallDetails<Z, F, I>
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
  gui_release_hash: String,              // ActionHash of the GUIReleaseEntry Record
) -> Result<Vec<u8>, String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call tauri command 'fetch_gui' which is not allowed in this window."));
  }

  let agent_pub_key = AgentPubKey::from(
    AgentPubKeyB64::from_b64_str(appstore_pub_key.as_str()).map_err(|e| {
      format!(
        "Failed to convert appstorePubKey from Base64 to Vec<u8>: {}",
        e
      )
    })?,
  );
  let gui_release_action_hash = ActionHash::from(
    ActionHashB64::from_b64_str(gui_release_hash.as_str()).map_err(|e| {
      format!(
        "Failed to convert guiReleaseHash from Base64 to Vec<u8>: {}",
        e
      )
    })?,
  );

  let mut ws = AppWebsocket::connect(format!("ws://localhost:{}", app_port))
    .await
    .map_err(|e| format!("Failed to connect to app websocket: {}", e))?;

  let app_info: AppInfo = ws
    .app_info(appstore_app_id.clone())
    .await
    .map_err(|e| format!("Failed to get appstore AppInfo: {:?}", e))?
    .ok_or(format!("AppInfo is None."))?;

  let cells = app_info
    .cell_info
    .get("portal")
    .ok_or(format!("No CellInfo found for portal role"))?;

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
      id: gui_release_action_hash,
    },
  )
  .await?;

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
    },
  )
  .await?;

  let ui_bytes = fetch_mere_memory(
    &mut ws,
    lair_keystore_manager,
    &agent_pub_key,
    host.clone(),
    portal_cell,
    "web_assets",
    devhub_happ_library_dna_hash.clone(),
    web_asset_file.content.mere_memory_addr,
  )
  .await?;

  Ok(ui_bytes)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebHappBundle {
  pub manifest: WebHappManifest,
  pub resources: BTreeMap<String, Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HappBundle {
  pub manifest: HappManifest,
  pub resources: BTreeMap<String, Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DnaBundle {
  pub manifest: Manifest,
  pub resources: BTreeMap<String, Vec<u8>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Manifest {
  pub manifest_version: String,
  pub name: String,
  pub integrity: IntegrityZomes,
  pub coordinator: CoordinatorZomes,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebHappManifest {
  pub manifest_version: String,
  pub name: String,
  pub ui: ResourceRef,
  pub happ_manifest: ResourceRef,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourceRef {
  pub bundled: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BundleIntegrityZomeInfo {
  pub name: String,
  pub bundled: String,
  // Optional fields
  pub hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DependencyRef {
  pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BundleZomeInfo {
  pub name: String,
  pub bundled: String,
  pub dependencies: Vec<DependencyRef>,

  // Optional fields
  pub hash: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IntegrityZomes {
  origin_time: HumanTimestamp,
  zomes: Vec<BundleIntegrityZomeInfo>,

  // Optional fields
  pub network_seed: Option<String>,
  pub properties: Option<BTreeMap<String, serde_yaml::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CoordinatorZomes {
  zomes: Vec<BundleZomeInfo>,
}

/// Fetch and assemble a happ from a devhub host
async fn fetch_and_assemble_web_happ(
  app_websocket: &mut AppWebsocket,
  lair_keystore_manager: &Box<dyn LairKeystoreManager>,
  agent_pub_key: &AgentPubKey,
  host: AgentPubKey,
  portal_cell: &ProvisionedCell,
  devhub_happ_library_dna_hash: DnaHash,
  happ_release_entry: HappReleaseEntry,
  name: String,
) -> Result<Vec<u8>, String> {
  if happ_release_entry.official_gui.is_none() {
    return Err(String::from(
      "Failed to fetch and assemble webhapp: The official_gui of the passed is 'None'.",
    ));
  }

  let gui_release_entry_entity: Entity<GUIReleaseEntry> = portal_remote_call(
    app_websocket,
    lair_keystore_manager,
    agent_pub_key,
    host.clone(),
    portal_cell,
    devhub_happ_library_dna_hash.clone(),
    String::from("happ_library"),
    String::from("get_gui_release"),
    GetEntityInput {
      id: happ_release_entry.official_gui.clone().unwrap(),
    },
  )
  .await?;

  let web_asset_file: Entity<FileEntry> = portal_remote_call(
    app_websocket,
    lair_keystore_manager,
    agent_pub_key,
    host.clone(),
    portal_cell,
    devhub_happ_library_dna_hash.clone(),
    String::from("happ_library"),
    String::from("get_webasset_file"),
    GetEntityInput {
      id: gui_release_entry_entity.content.web_asset_id,
    },
  )
  .await?;

  let ui_bytes = fetch_mere_memory(
    app_websocket,
    lair_keystore_manager,
    agent_pub_key,
    host.clone(),
    portal_cell,
    "web_assets",
    devhub_happ_library_dna_hash.clone(),
    web_asset_file.content.mere_memory_addr,
  )
  .await?;

  let happ_bundle_bytes = fetch_and_assemble_happ(
    app_websocket,
    lair_keystore_manager,
    agent_pub_key,
    host,
    portal_cell,
    devhub_happ_library_dna_hash,
    happ_release_entry,
  )
  .await?;

  let mut resources: BTreeMap<String, Vec<u8>> = BTreeMap::new();

  let ui_ref = String::from("./ui.zip");
  resources.insert(ui_ref.clone(), ui_bytes);

  let happ_ref = String::from("./bundle.happ");
  resources.insert(happ_ref.clone(), happ_bundle_bytes);

  let web_happ_bundle = WebHappBundle {
    manifest: WebHappManifest {
      manifest_version: String::from("1"),
      name,
      ui: ResourceRef { bundled: ui_ref },
      happ_manifest: ResourceRef { bundled: happ_ref },
    },
    resources: resources,
  };

  let web_happ_bundle_bytes = encode_bundle(web_happ_bundle)
    .map_err(|e| format!("Failed to encode webhapp bundle: {}", e))?;

  Ok(web_happ_bundle_bytes)
}

/// Fetch and assemble a happ from a devhub host
pub async fn fetch_and_assemble_happ(
  app_websocket: &mut AppWebsocket,
  lair_keystore_manager: &Box<dyn LairKeystoreManager>,
  agent_pub_key: &AgentPubKey,
  host: AgentPubKey,
  portal_cell: &ProvisionedCell,
  devhub_happ_library_dna_hash: DnaHash,
  mut happ_release_entry: HappReleaseEntry,
) -> Result<Vec<u8>, String> {
  // 1. Get all .dna files
  let mut dna_resources: BTreeMap<String, Vec<u8>> = BTreeMap::new();

  for (i, dna_ref) in happ_release_entry.dnas.iter().enumerate() {
    let dna_path = format!("./{}.dna", dna_ref.role_name);

    println!(
      "Assembling data for dna with role_name: {}",
      dna_ref.role_name
    );
    println!("DNA path: {}", dna_path);

    let dna_version: Entity<DnaVersionEntry> = portal_remote_call(
      app_websocket,
      lair_keystore_manager,
      agent_pub_key,
      host.clone(),
      portal_cell,
      devhub_happ_library_dna_hash.clone(),
      String::from("happ_library"),
      String::from("get_dna_version"),
      GetEntityInput {
        id: dna_ref.version.to_owned(),
      },
    )
    .await?;

    let mut resources: BTreeMap<String, Vec<u8>> = BTreeMap::new();
    let mut integrity_zomes: Vec<BundleIntegrityZomeInfo> = vec![];
    let mut coordinator_zomes: Vec<BundleZomeInfo> = vec![];

    for zome_ref in dna_version.content.integrity_zomes {
      let wasm_bytes = fetch_mere_memory(
        app_websocket,
        lair_keystore_manager,
        agent_pub_key,
        host.clone(),
        portal_cell,
        "dnarepo",
        devhub_happ_library_dna_hash.clone(),
        zome_ref.resource,
      )
      .await?;

      let path = format!("./{}.wasm", zome_ref.name);

      integrity_zomes.push(BundleIntegrityZomeInfo {
        name: zome_ref.name.clone(),
        bundled: path.clone(),
        hash: None,
      });

      resources.insert(path, wasm_bytes);
    }

    for zome_ref in dna_version.content.zomes {
      let wasm_bytes = fetch_mere_memory(
        app_websocket,
        lair_keystore_manager,
        agent_pub_key,
        host.clone(),
        portal_cell,
        "dnarepo",
        devhub_happ_library_dna_hash.clone(),
        zome_ref.resource,
      )
      .await?;

      let path = format!("./{}.wasm", zome_ref.name);

      coordinator_zomes.push(BundleZomeInfo {
        name: zome_ref.name.clone(),
        bundled: path.clone(),
        hash: None,
        dependencies: zome_ref
          .dependencies
          .iter()
          .map(|name| DependencyRef {
            name: name.to_owned(),
          })
          .collect(),
      });

      resources.insert(path, wasm_bytes);
    }

    let dna_bundle = DnaBundle {
      manifest: Manifest {
        manifest_version: "1".into(),
        name: dna_ref.role_name.clone(),
        integrity: IntegrityZomes {
          origin_time: dna_version.content.origin_time.clone(),
          network_seed: dna_version.content.network_seed.clone(),
          properties: dna_version.content.properties.clone(),
          zomes: integrity_zomes,
        },
        coordinator: CoordinatorZomes {
          zomes: coordinator_zomes,
        },
      },
      resources: resources,
    };

    let dna_pack_bytes = encode_bundle(dna_bundle).map_err(|e| {
      format!(
        "Failed to encode bundle for dna {}: {}",
        dna_ref.role_name, e
      )
    })?;

    dna_resources.insert(dna_path.clone(), dna_pack_bytes);
    happ_release_entry.manifest.roles[i].dna.bundled = dna_path;
  }

  println!("happ manifest: {:?}", happ_release_entry.manifest);
  println!("dna_resources keys: {:?}", dna_resources.keys());

  let happ_bundle = HappBundle {
    manifest: happ_release_entry.manifest,
    resources: dna_resources,
  };

  let happ_pack_bytes =
    encode_bundle(happ_bundle).map_err(|e| format!("Failed to encode happ bundle: {}", e))?;

  Ok(happ_pack_bytes)
}

/// Fetching and combining bytes by mere_memory_address
pub async fn fetch_mere_memory(
  app_websocket: &mut AppWebsocket,
  lair_keystore_manager: &Box<dyn LairKeystoreManager>,
  agent_pub_key: &AgentPubKey,
  host: AgentPubKey,
  portal_cell: &ProvisionedCell,
  dna_name: &str,
  devhub_happ_library_dna_hash: DnaHash,
  memory_address: EntryHash,
) -> Result<Vec<u8>, String> {
  // 1. get MemoryEntry
  let memory_entry: MemoryEntry = portal_remote_call(
    app_websocket,
    lair_keystore_manager,
    agent_pub_key,
    host.clone(),
    portal_cell,
    devhub_happ_library_dna_hash.clone(),
    String::from("happ_library"),
    format!("{}_get_memory", dna_name),
    memory_address,
  )
  .await?;

  let mut memory_blocks: Vec<MemoryBlockEntry> = Vec::new();
  // 2. Assemble all MemoryEntryBlock's
  for block_address in memory_entry.block_addresses {
    let memory_block_entry: MemoryBlockEntry = portal_remote_call(
      app_websocket,
      lair_keystore_manager,
      agent_pub_key,
      host.clone(),
      portal_cell,
      devhub_happ_library_dna_hash.clone(),
      String::from("happ_library"),
      format!("{}_get_memory_block", dna_name),
      block_address,
    )
    .await?;

    memory_blocks.push(memory_block_entry);
  }

  // 3. Sort and combine them
  memory_blocks.sort_by(|a, b| a.sequence.position.cmp(&b.sequence.position));

  let combined_memory = memory_blocks
    .into_iter()
    .map(|m| m.bytes)
    .flatten()
    .collect::<Vec<u8>>();

  Ok(combined_memory)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomRemoteCallInput<T: Serialize + core::fmt::Debug> {
  host: AgentPubKey,
  call: RemoteCallDetails<String, String, T>,
}

/// Wrapper for remote calls through the portal_api
pub async fn portal_remote_call<
  T: Serialize + core::fmt::Debug,
  U: Serialize + DeserializeOwned + core::fmt::Debug,
>(
  app_websocket: &mut AppWebsocket,
  lair_keystore_manager: &Box<dyn LairKeystoreManager>,
  agent_pub_key: &AgentPubKey,
  host: AgentPubKey,
  portal_cell: &ProvisionedCell,
  dna: DnaHash,
  zome: String,
  function: String,
  payload: T,
) -> Result<U, String> {
  let (nonce, expires_at) =
    fresh_nonce(Timestamp::now()).map_err(|e| format!("Failed to create fresh Nonce: {:?}", e))?;

  let input = CustomRemoteCallInput {
    host,
    call: RemoteCallDetails {
      dna,
      zome: zome.clone(),
      function: function.clone(),
      payload,
    },
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

  let result = app_websocket
    .call_zome(signed_zome_call)
    .await
    .map_err(|e| format!("Zome call failed: {:?}", e))?;

  let response: DevHubResponse<DevHubResponse<U>> = result.decode().map_err(|e| {
    format!(
      "Error decoding the remote call response for zome '{}' and function '{}': {}",
      zome, function, e
    )
  })?;

  let inner_response = match response {
    DevHubResponse::Success(pack) => pack.payload,
    DevHubResponse::Failure(error) => {
      println!("Errorpayload: {:?}", error.payload);
      return Err(format!("Received ErrorPayload: {:?}", error.payload));
    }
  };

  let bytes = inner_response
    .as_result()
    .map_err(|e| format!("Failed to get content from DevHubResponse: {}", e))?;

  Ok(bytes)
}
