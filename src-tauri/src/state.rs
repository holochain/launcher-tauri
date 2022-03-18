use std::sync::{Arc, Mutex};

use crate::{
  connection_status::ConnectionStatus,
  managers::{
    holochain::{conductor::versions::V0_0_130::ConductorManagerV0_0_130, HolochainManager},
    launcher::LauncherManager,
  },
};

pub enum LauncherState {
  Running(Arc<Mutex<ConnectionStatus<LauncherManager>>>),
  AnotherInstanceIsAlreadyRunning,
}

impl LauncherState {
  pub fn get_holochain_manager(
    &self,
  ) -> Result<HolochainManager<ConductorManagerV0_0_130>, String> {
    if let LauncherState::Running(m) = self {
      if let ConnectionStatus::Connected(holochain_manager) = (*m.lock().unwrap()).holochain_manager
      {
        return Ok(holochain_manager);
      }
    }

    Err(String::from(
      "The requested Holochain manager is not running",
    ))
  }

  pub fn get_connection_status(&self) -> String {
    match self {
      LauncherState::Running(m) => match *m.holochain_manager.lock().unwrap() {
        ConnectionStatus::Connected(_) => String::from("connected"),
        ConnectionStatus::Error { error } => format!("err: {}", error),
      },
      _ => String::from("another_instance_exists"),
    }
  }
}
