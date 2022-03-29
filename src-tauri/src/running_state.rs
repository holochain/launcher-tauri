use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type", content = "content")]
pub enum RunningState<C, E> {
  Running(C),
  Error(E),
}
