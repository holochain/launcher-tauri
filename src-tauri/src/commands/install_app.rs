use holochain_manager::versions::{
  holochain_types_latest::{
    prelude::{AgentPubKey, AppBundle, MembraneProof, UnsafeBytes, SerializedBytes},
    web_app::WebAppBundle,
  },
};
use std::{collections::HashMap, fs, sync::Arc};

use crate::{launcher::{state::LauncherState, manager::HolochainId}, file_system::CustomPath};

#[tauri::command]
pub async fn install_app(
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  custom_path: tauri::State<'_, CustomPath>,
  holochain_id: HolochainId,
  app_id: String,
  app_bundle_path: String,
  network_seed: Option<String>,
  membrane_proofs: HashMap<String, Vec<u8>>,
  reuse_agent_pub_key: Option<AgentPubKey>,
) -> Result<(), String> {
  if window.label() != "admin" {
    return Err(String::from("Unauthorized: Attempted to call an unauthorized tauri command. (I)"))
  }

  log::info!("Installing: web_app_bundle = {}", app_bundle_path);

  let mut converted_membrane_proofs: HashMap<String, MembraneProof> = HashMap::new();
  for (dna_slot, proof) in membrane_proofs.iter() {
    converted_membrane_proofs.insert(
      dna_slot.clone(),
      Arc::new(SerializedBytes::from(UnsafeBytes::from(proof.clone()))),
    );
  }

  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  let bytes = fs::read(&app_bundle_path).or(Err("Failed to read Web hApp bundle file"))?;

  match WebAppBundle::decode(&bytes) {
    Ok(web_app_bundle) => {
      manager
        .get_or_launch_holochain(holochain_id, custom_path.custom_path.clone())
        .await?
        .install_web_app(
          app_id.clone(),
          web_app_bundle,
          network_seed,
          converted_membrane_proofs,
          reuse_agent_pub_key,
        )
        .await?;
    }
    Err(_) => {
      let app_bundle = AppBundle::decode(&bytes).or(Err("Failed to read Web hApp bundle file"))?;
      manager
        .get_or_launch_holochain(holochain_id, custom_path.custom_path.clone())
        .await?
        .install_app(
          app_id.clone(),
          app_bundle,
          network_seed,
          converted_membrane_proofs,
          reuse_agent_pub_key,
        )
        .await?;
    }
  }

  log::info!("Installed hApp {}", app_id);

  manager.on_apps_changed().await?;

  Ok(())
}
