use holochain_manager::versions::{version_manager::VersionManager, HdkVersion, HolochainVersion};
use serde::{Deserialize, Serialize};

use crate::{launcher::state::LauncherState, running_state::RunningState};

#[tauri::command]
pub async fn choose_version_for_hdk(
  state: tauri::State<'_, LauncherState>,
  hdk_version: HdkVersion,
) -> Result<HolochainVersion, String> {
  let mut mutex = (*state).lock().await;
  let manager = mutex.get_running()?;

  for (version, state) in &manager.holochain_managers {
    if let RunningState::Running(_) = state {
      if version.manager().hdk_version().eq(&hdk_version) {
        return Ok(version.clone());
      }
    }
  }

  for version in HolochainVersion::supported_versions() {
    if version.manager().hdk_version().eq(&hdk_version) {
      return Ok(version.clone());
    }
  }

  Err(String::from(
    "There are no supported Holochain versions for this HDK version",
  ))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SupportedVersions {
  hdk_versions: Vec<HdkVersion>,
  holochain_versions: Vec<HolochainVersion>,
}

#[tauri::command]
pub fn get_supported_versions() -> SupportedVersions {
  let holochain_versions = HolochainVersion::supported_versions();

  let hdk_versions = holochain_versions
    .iter()
    .map(|v| v.manager().hdk_version())
    .collect();

  SupportedVersions {
    holochain_versions,
    hdk_versions,
  }
}
