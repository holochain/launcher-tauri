use futures::lock::Mutex;
use std::{collections::HashMap, sync::Arc};

use crate::{managers::launcher::LauncherManager, running_state::RunningState};
use holochain_versions::HolochainVersion;

pub type LauncherState = RunningState<Arc<Mutex<LauncherManager>>, RunLauncherError>;

#[derive(Clone)]
pub enum RunLauncherError {
  AnotherInstanceIsAlreadyRunning,
}

pub type LauncherStateInfo = RunningState<
  HashMap<HolochainVersion, RunningState<HolochainStateInfo, RunHolochainError>>,
  RunLauncherError,
>;

impl LauncherState {
  pub fn get_launcher_manager(&self) -> Result<&Arc<Mutex<LauncherManager>>, String> {
    match self {
      RunningState::Running(launcher_manager) => Ok(launcher_manager),
      _ => Err(String::from("The LauncherManager is not running")),
    }
  }

  pub async fn get_connection_status(&self) -> String {
    match self {
      LauncherState::Running(m) => match &m.lock().await.holochain_manager {
        ConnectionStatus::Connected(_) => String::from("connected"),
        ConnectionStatus::Error { error } => format!("err: {}", error),
      },
      LauncherState::ErrorLaunching { error } => {
        format!("There was an error launching holochain: {}", error)
      }
      _ => String::from("another_instance_exists"),
    }
  }

  pub async fn get_info(&self) -> LauncherStateInfo {
    match self {
      RunningState::Running(manager) => &manager.lock().await.holochain_manager,
      RunningState::Error(err) => LauncherStateInfo::Error(err.clone()),
    }
  }
}
