use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "type", content = "content")]
pub enum RunLauncherError {
  AnotherInstanceIsAlreadyRunning,
  OldFilesExist,
  ErrorLaunching(String),
}
