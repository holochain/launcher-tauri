use std::sync::{Arc, Mutex};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RunningPorts {
  pub admin_interface_port: u16,
  pub caddy_admin_port: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ConnectionStatus {
  // Normal state
  Connected(RunningPorts),
  // There was an error running the launcher
  Error { error: String },
  // There was already an older instance of the launcher running
  AlreadyRunning,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LauncherState {
  pub connection_status: Arc<Mutex<ConnectionStatus>>,
}

impl LauncherState {
  pub fn get_running_ports(&self) -> Result<RunningPorts, String> {
    match self.connection_status.lock().unwrap().clone() {
      ConnectionStatus::Connected(ports) => Ok(ports),
      _ => Err(String::from("The conductor is not running")),
    }
  }
}
