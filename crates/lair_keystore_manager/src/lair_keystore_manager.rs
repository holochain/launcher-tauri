use std::path::PathBuf;

use crate::{error::LaunchTauriSidecarError, versions::LairKeystoreVersion};
use async_trait::async_trait;

#[async_trait]
pub trait LairKeystoreManager: Sized {
  fn lair_keystore_version() -> LairKeystoreVersion;

  async fn launch(
    log_level: log::Level,
    keystore_path: PathBuf,
  ) -> Result<Self, LaunchTauriSidecarError>;
}
