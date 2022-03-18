use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "type")]
pub enum ConnectionStatus<C> {
  // Connected state
  Connected(C),
  // There was an error connecting
  Error { error: String },
}
