use async_recursion::async_recursion;
use std::collections::HashMap;

use holochain_manager::versions::{HolochainVersion, version_manager::VersionManager};
use holochain_web_app_manager::WebAppManager;

use crate::{
  launcher::{
    config::LauncherConfig,
    error::LauncherError,
    manager::{HolochainId, KeystoreStatus, LauncherManager},
    state::{
      HolochainInfo, HolochainState, LauncherState, LauncherStateInfo, RunningHolochainsStateInfo,
    },
  },
  running_state::RunningState, file_system::Profile,
};

#[tauri::command]
pub async fn get_state_info(
  window: tauri::Window,
  state: tauri::State<'_, LauncherState>,
  profile: tauri::State<'_, Profile>,
) -> Result<LauncherStateInfo, LauncherError> {
  if window.label() != "admin" {
    return Err(LauncherError::Unauthorized("Attempted to call an unauthorized tauri command. (G)".into()))
  }

  let state_info = inner_get_state_info(state).await?;

  let config = LauncherConfig::read(profile.inner().clone());

  Ok(LauncherStateInfo {
    state: state_info,
    config,
    default_version: HolochainVersion::default()
  })
}

async fn inner_get_state_info(
  state: tauri::State<'_, LauncherState>,
) -> Result<
  RunningState<RunningState<RunningHolochainsStateInfo, KeystoreStatus>, LauncherError>,
  LauncherError,
> {
  let mut mutex = state.lock().await;

  let state: &mut RunningState<LauncherManager, LauncherError> = &mut *mutex;

  match state {
    RunningState::Running(manager) => {
      if let RunningState::Error(err) = &manager.lair_keystore_manager {
        return Ok(RunningState::Running(RunningState::Error(err.clone())));
      }

      let mut holochain_manager_states: HashMap<HolochainVersion, HolochainState> = HashMap::new();

      let versions: Vec<HolochainVersion> = manager.holochain_managers.keys().cloned().collect();

      for holochain_version in versions {
        match manager.get_web_happ_manager(HolochainId::HolochainVersion(holochain_version.clone()))
        {
          Ok(holochain_manager) => {
            let running_state = get_holochain_state(holochain_manager, 2).await;
            holochain_manager_states.insert(holochain_version.clone(), running_state);
          }
          Err(err) => {
            holochain_manager_states
              .insert(holochain_version.clone(), RunningState::Error(err.clone()));
          }
        }
      }

      let custom_binary = match &mut manager.custom_binary_manager {
        Some(RunningState::Running(m)) => Some(get_holochain_state(m, 2).await),
        Some(RunningState::Error(err)) => Some(RunningState::Error(format!(
          "There was an error launching the custom Holochain binary: {:?}",
          err
        ))),
        None => None,
      };

      Ok(RunningState::Running(RunningState::Running(
        RunningHolochainsStateInfo {
          versions: holochain_manager_states,
          custom_binary,
        },
      )))
    }
    RunningState::Error(err) => Ok(RunningState::Error(err.clone())),
  }
}

#[async_recursion]
async fn get_holochain_state(web_app_manager: &mut WebAppManager, retries: usize) -> HolochainState {
  match web_app_manager.list_apps().await {
    Ok(installed_apps) => RunningState::Running(HolochainInfo {
      installed_apps,
      app_interface_port: web_app_manager.app_interface_port(),
      admin_interface_port: web_app_manager.admin_interface_port(),
      hdi_version: web_app_manager.holochain_manager.version.manager().hdi_version(),
      hdk_version: web_app_manager.holochain_manager.version.manager().hdk_version(),
    }),
    Err(err) => {
      match retries {
        0 => RunningState::Error(format!("Could not fetch installed apps: {}", err)),
        _ => get_holochain_state(web_app_manager, retries - 1).await,
      }
    }
  }
}
