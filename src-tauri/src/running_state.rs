use std::fmt::Debug;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "content")]
pub enum RunningState<C, E: Debug> {
  Running(C),
  Error(E),
}

impl<C, E: Debug> RunningState<C, E> {
  pub fn get_running(&mut self) -> Result<&mut C, String> {
    match self {
      RunningState::Running(c) => Ok(c),
      RunningState::Error(e) => Err(format!("The requested resource is not running: {:?}", e)),
    }
  }
}
