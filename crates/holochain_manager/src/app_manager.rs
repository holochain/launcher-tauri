use std::collections::HashMap;

use async_trait::async_trait;

use crate::versions::holochain_types_latest::prelude::*;

#[async_trait]
pub trait AppManager {
  type RunningApps;

  async fn install_app(
    &mut self,
    app_id: String,
    app_bundle: AppBundle,
    uid: Option<String>,
    membrane_proofs: HashMap<String, SerializedBytes>,
  ) -> Result<(), String>;

  async fn uninstall_app(&mut self, app_id: String) -> Result<(), String>;

  async fn enable_app(&mut self, app_id: String) -> Result<(), String>;

  async fn disable_app(&mut self, app_id: String) -> Result<(), String>;

  async fn get_running_apps(&mut self) -> Result<Self::RunningApps, String>;
}
