use std::collections::HashMap;

use async_trait::async_trait;
use enum_dispatch::enum_dispatch;
use lair_keystore_manager::versions::LairKeystoreVersion;

use crate::versions::holochain_conductor_api_latest::InstalledAppInfo;
use crate::versions::holochain_types_latest::prelude::*;
use crate::versions::HolochainVersion;

#[async_trait]
#[enum_dispatch]
pub trait HolochainManager: Send + Sync {
  fn holochain_version(&self) -> HolochainVersion;

  fn lair_keystore_version(&self) -> LairKeystoreVersion;

  fn app_interface_port(&self) -> u16;

  fn admin_interface_port(&self) -> u16;

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

  async fn list_apps(&mut self) -> Result<Vec<InstalledAppInfo>, String>;

  fn kill(self) -> Result<(), String>;
}
