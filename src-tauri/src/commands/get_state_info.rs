use std::collections::HashMap;

use holochain_manager::versions::HolochainVersion;

use crate::{
  launcher::{
    config::LauncherConfig,
    error::LauncherError,
    manager::{KeystoreStatus, LauncherManager},
    state::{HolochainInfo, HolochainState, LauncherState, LauncherStateInfo},
  },
  running_state::RunningState,
};

#[tauri::command]
pub async fn get_state_info(
  state: tauri::State<'_, LauncherState>,
) -> Result<LauncherStateInfo, LauncherError> {
  let state_info = inner_get_state_info(state).await?;

  let config = LauncherConfig::read();

  Ok(LauncherStateInfo {
    state: state_info,
    config,
  })
}

async fn inner_get_state_info(
  state: tauri::State<'_, LauncherState>,
) -> Result<
  RunningState<
    RunningState<HashMap<HolochainVersion, HolochainState>, KeystoreStatus>,
    LauncherError,
  >,
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
        match manager.get_web_happ_manager(holochain_version.clone()) {
          Ok(holochain_manager) => {
            let running_state = match holochain_manager.list_apps().await {
              Ok(installed_apps) => RunningState::Running(HolochainInfo {
                installed_apps,
                app_interface_port: holochain_manager.app_interface_port(),
                admin_interface_port: holochain_manager.admin_interface_port(),
              }),
              Err(err) => RunningState::Error(format!("Could not fetch installed apps: {}", err)),
            };

            holochain_manager_states.insert(holochain_version.clone(), running_state);
          }
          Err(err) => {
            holochain_manager_states
              .insert(holochain_version.clone(), RunningState::Error(err.clone()));
          }
        }
      }

      Ok(RunningState::Running(RunningState::Running(
        holochain_manager_states,
      )))
    }
    RunningState::Error(err) => Ok(RunningState::Error(err.clone())),
  }
}
