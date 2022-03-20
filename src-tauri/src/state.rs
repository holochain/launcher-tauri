use futures::lock::Mutex;
use std::sync::Arc;

use crate::{connection_status::ConnectionStatus, managers::launcher::LauncherManager};

pub enum LauncherState {
  Running(Arc<Mutex<LauncherManager>>),
  ErrorLaunching { error: String },
  AnotherInstanceIsAlreadyRunning,
}

impl LauncherState {
  pub fn get_launcher_manager(&self) -> Result<&Arc<Mutex<LauncherManager>>, String> {
    match self {
      LauncherState::Running(m) => Ok(m),
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
}
