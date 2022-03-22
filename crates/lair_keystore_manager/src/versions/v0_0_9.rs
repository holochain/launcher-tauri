use std::path::PathBuf;

use async_trait::async_trait;

use super::{launch::launch_lair_keystore_process, LairKeystoreVersion};
use crate::{LairKeystoreManager, error::LaunchLairKeystoreError};

pub struct LairKeystoreManagerV0_0_9 {
    _keystore_path: PathBuf,
}

#[async_trait]
impl LairKeystoreManager for LairKeystoreManagerV0_0_9 {
    fn lair_keystore_version() -> LairKeystoreVersion {
        LairKeystoreVersion::V0_0_9
    }

    async fn launch(log_level: log::Level, keystore_path: PathBuf) -> Result<Self, LaunchLairKeystoreError> {
        launch_lair_keystore_process(log_level, keystore_path.clone())?;

        Ok(LairKeystoreManagerV0_0_9 {
            _keystore_path: keystore_path,
        })
    }
}
