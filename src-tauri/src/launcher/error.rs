use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type")]
pub enum RunLauncherError {
  AnotherInstanceIsAlreadyRunning,
  ErrorLaunching(String),
}
