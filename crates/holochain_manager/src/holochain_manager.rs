use std::collections::HashMap;
use std::path::PathBuf;

use async_trait::async_trait;

use crate::error::LaunchHolochainError;
use crate::versions::holochain_types_latest::prelude::*;
use crate::versions::HolochainVersion;

#[async_trait]
pub trait HolochainManager {
    /// Launch the conductor
    /// If there already was an instantiated conductor in the given paths, use those folders
    /// If not, create the necessary files and folder to start afresh
    async fn launch(
        log_level: log::Level,
        admin_port: u16,
        conductor_config_path: PathBuf,
        environment_path: PathBuf,
        keystore_path: PathBuf,
    ) -> Result<Self, LaunchHolochainError>
    where
        Self: Sized;

    fn holochain_version() -> HolochainVersion
    where
        Self: Sized;

    async fn get_app_port(&mut self) -> Result<u16, String>;

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

    async fn list_running_apps(&mut self) -> Result<Vec<String>, String>;
}
