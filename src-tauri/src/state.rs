use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ConnectionStatus {
  // Normal state
  Connected { admin_interface_port: u16 },
  // There was an error running the launcher
  Error { error: String },
  // There was already an older instance of the launcher running
  AlreadyRunning,
}

impl ConnectionStatus {
  pub fn get_admin_port(&self) -> Result<u16, String> {
    match self {
      ConnectionStatus::Connected {
        admin_interface_port,
      } => Ok(admin_interface_port.clone()),
      _ => Err(String::from("Launcher is not connected")),
    }
  }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LauncherState {
  pub connection_status: ConnectionStatus,
}
