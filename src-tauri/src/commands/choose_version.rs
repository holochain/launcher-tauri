use holochain_manager::versions::{version_manager::VersionManager, HdkVersion, HolochainVersion};

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

#[tauri::command]
pub fn get_supported_hdk_versions() -> Vec<HdkVersion> {
  HolochainVersion::supported_versions()
    .into_iter()
    .map(|v| v.manager().hdk_version())
    .collect()
}
