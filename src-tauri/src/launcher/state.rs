use futures::lock::Mutex;
use holochain_manager::{app_manager::AppManager, versions::HolochainVersion};
use holochain_web_app_manager::running_apps::RunningApps;
use std::{collections::HashMap, sync::Arc};

use super::{error::RunLauncherError, manager::LauncherManager};
use crate::running_state::RunningState;

pub type LauncherState = RunningState<Arc<Mutex<LauncherManager>>, RunLauncherError>;

pub type HolochainStateInfo = RunningState<RunningApps, String>;

pub type LauncherStateInfo =
  RunningState<HashMap<HolochainVersion, HolochainStateInfo>, RunLauncherError>;

impl LauncherState {
  pub fn get_launcher_manager(&self) -> Result<&Arc<Mutex<LauncherManager>>, String> {
    match self {
      RunningState::Running(launcher_manager) => Ok(launcher_manager),
      _ => Err(String::from("The LauncherManager is not running")),
    }
  }

  pub async fn get_state_info(&self) -> Result<LauncherStateInfo, String> {
    match self {
      RunningState::Running(manager_mutex) => {
        let mut manager = manager_mutex.lock().await;

        let mut holochain_manager_states: HashMap<HolochainVersion, HolochainStateInfo> =
          HashMap::new();

        let versions: Vec<HolochainVersion> = manager.holochain_managers.keys().cloned().collect();

        for holochain_version in versions {
          match manager.get_web_happ_manager(holochain_version.clone()) {
            Ok(holochain_manager) => {
              let running_apps = holochain_manager.get_running_apps().await?;

              holochain_manager_states.insert(
                holochain_version.clone(),
                RunningState::Running(running_apps),
              );
            }
            Err(err) => {
              holochain_manager_states
                .insert(holochain_version.clone(), RunningState::Error(err.clone()));
            }
          }
        }

        Ok(RunningState::Running(holochain_manager_states))
      }
      RunningState::Error(err) => Ok(RunningState::Error(err.clone())),
    }
  }
}
