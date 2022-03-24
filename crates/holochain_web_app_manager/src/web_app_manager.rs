use holochain_manager::{
  versions::{
    holochain_types_latest::{
      prelude::{AppBundle, SerializedBytes},
      web_app::WebAppBundle,
    },
    launch_holochain,
    mr_bundle_latest::ResourceBytes,
    utils::create_dir_if_necessary,
    HolochainVersion,
  },
  HolochainManager,
};
use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  fs::{self, File},
  path::{Path, PathBuf},
};

use crate::utils::unzip_file;
use crate::error::LaunchWebAppManagerError;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ManagedApp {
  Headless,
  WebApp { path_to_web_app: PathBuf },
}

pub struct WebAppManager {
  root_path: PathBuf,
  holochain_manager: Box<dyn HolochainManager>,
  pub admin_port: u16,
  pub app_interface_port: u16,
}

impl WebAppManager {
  pub async fn launch(
    version: HolochainVersion,
    log_level: log::Level,
    admin_port: u16,
    conductor_config_path: PathBuf,
    environment_path: PathBuf,
    keystore_path: PathBuf,
  ) -> Result<Self, LaunchWebAppManagerError> {
    let conductor_data_path = environment_path.join("conductor");
    let ui_data_path = uis_data_path(environment_path.clone());

    create_dir_if_necessary(&conductor_data_path);
    create_dir_if_necessary(&ui_data_path);

    let mut holochain_manager = launch_holochain(
      version,
      log_level,
      admin_port,
      conductor_config_path,
      conductor_data_path,
      keystore_path,
    )
    .await.map_err(|err| LaunchWebAppManagerError::LaunchHolochainError(err))?;

    let app_interface_port = holochain_manager.get_app_interface_port().await.map_err(|err| LaunchWebAppManagerError::CouldNotGetAppPort(err))?;

    Ok(WebAppManager {
      holochain_manager,
      root_path: environment_path,
      admin_port,
      app_interface_port,
    })
  }

  pub async fn install_headless_app(
    &mut self,
    app_id: String,
    app_bundle: AppBundle,
    uid: Option<String>,
    membrane_proofs: HashMap<String, SerializedBytes>,
  ) -> Result<(), String> {
    // Install app in conductor manager
    self
      .holochain_manager
      .install_app(app_id, app_bundle, uid, membrane_proofs)
      .await
      .map_err(|err| {
        log::error!("Error installing hApp in the conductor: {}", err);
        err
      })?;

    Ok(())
  }

  pub async fn install_app(
    &mut self,
    app_id: String,
    web_app_bundle: WebAppBundle,
    uid: Option<String>,
    membrane_proofs: HashMap<String, SerializedBytes>,
  ) -> Result<(), String> {
    let app_bundle = web_app_bundle
      .happ_bundle()
      .await
      .or(Err("Failed to resolve hApp bundle"))?;

    self
      .install_headless_app(app_id.clone(), app_bundle, uid, membrane_proofs)
      .await?;

    // Install app in UI manager

    let web_ui_zip_bytes = web_app_bundle
      .web_ui_zip_bytes()
      .await
      .or(Err("Failed to resolve Web UI"))?;

    self
      .install_app_ui(app_id, web_ui_zip_bytes.to_vec())
      .map_err(|err| {
        log::error!("Error installing the UI for hApp: {}", err);
        err
      })?;

    Ok(())
  }

  pub async fn uninstall_app(&mut self, app_id: String) -> Result<(), String> {
    self
      .holochain_manager
      .uninstall_app(app_id.clone())
      .await
      .map_err(|err| {
        log::error!("Error uninstalling hApp in the conductor: {}", err);
        err
      })?;

    self.uninstall_app_ui(app_id)?;

    Ok(())
  }

  pub async fn get_running_apps(&mut self) -> Result<HashMap<String, ManagedApp>, String> {
    let active_apps = self.holochain_manager.list_running_apps().await?;

    let mut running_apps_map: HashMap<String, ManagedApp> = HashMap::new();

    for app_id in active_apps {
      let ui_folder_path = app_ui_path(self.root_path.clone(), app_id.clone());

      let running_app = match Path::new(&ui_folder_path).exists() {
        true => ManagedApp::WebApp {
          path_to_web_app: ui_folder_path,
        },
        false => ManagedApp::Headless,
      };
      running_apps_map.insert(app_id, running_app);
    }

    Ok(running_apps_map)
  }

  fn install_app_ui(&self, app_id: String, web_ui_zip_bytes: ResourceBytes) -> Result<(), String> {
    let ui_folder_path = app_ui_path(self.root_path.clone(), app_id.clone());
    let ui_zip_path = uis_data_path(self.root_path.clone()).join(format!("{}.zip", app_id));

    fs::write(ui_zip_path.clone(), web_ui_zip_bytes).or(Err("Failed to write Web UI Zip file"))?;

    let file = File::open(ui_zip_path).or(Err("Failed to read Web UI Zip file"))?;
    unzip_file(file, ui_folder_path)?;

    Ok(())
  }

  fn uninstall_app_ui(&self, app_id: String) -> Result<(), String> {
    let ui_folder_path = app_ui_path(self.root_path.clone(), app_id);

    if Path::new(&ui_folder_path).exists() {
      fs::remove_dir_all(ui_folder_path).or(Err("Failed to remove UI folder"))?;
    }

    Ok(())
  }
}

fn uis_data_path(root_path: PathBuf) -> PathBuf {
  root_path.join("uis")
}

fn app_ui_path(root_path: PathBuf, app_id: String) -> PathBuf {
  uis_data_path(root_path).join(app_id)
}
