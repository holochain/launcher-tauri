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
  pub fn get_launcher_manager(&self) -> Result<LauncherManager, String> {
    match self {
      LauncherState::Running(m) => match *m.lock().unwrap() {
        ConnectionStatus::Connected(manager) => Ok(manager),
        ConnectionStatus::Error { error } => Err(error),
      },
      _ => Err(String::from("The LauncherManager is not running")),
    }
  }

  pub fn get_holochain_manager(
    &self,
  ) -> Result<HolochainManager<ConductorManagerV0_0_130>, String> {
    let launcher_manager = self.get_launcher_manager()?;

    Ok(launcher_manager.holochain_manager)
  }

  pub fn get_connection_status(&self) -> String {
    match self {
      LauncherState::Running(m) => match *m.lock().unwrap() {
        ConnectionStatus::Connected(_) => String::from("connected"),
        ConnectionStatus::Error { error } => format!("err: {}", error),
      },
      _ => String::from("another_instance_exists"),
    }
  }
}
