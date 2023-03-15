use holochain_conductor_api_0_1_3::conductor::ConductorConfig;
use holochain_p2p_0_1_3::kitsune_p2p::dependencies::kitsune_p2p_types::dependencies::lair_keystore_api::dependencies::sodoken::BufRead;
use std::fs;
use std::path::PathBuf;

use holochain::conductor::{Conductor, ConductorHandle};

use crate::errors::LaunchHolochainError;

pub async fn launch_holochain_process(
  conductor_config_path: PathBuf,
  password: String,
) -> Result<ConductorHandle, LaunchHolochainError> {
  let conductor_config = fs::read_to_string(conductor_config_path)?;
  let config: ConductorConfig = serde_yaml::from_str(conductor_config.as_str())
    .map_err(|err| LaunchHolochainError::Serde(format!("{:?}", err)))?;
  Ok(
    Conductor::builder()
      .config(config)
      .passphrase(Some(BufRead::new_no_lock(password.as_bytes())))
      .build()
      .await
      .map_err(|err| LaunchHolochainError::ConductorError(format!("{:?}", err)))?,
  )
}
