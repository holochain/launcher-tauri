use holochain_manager::{
  config::LaunchHolochainConfig,
  versions::{
    holochain_conductor_api_latest::AppInfo,
    holochain_types_latest::{
      prelude::{AgentPubKey, AppBundle, MembraneProof},
      web_app::WebAppBundle,
    },
    mr_bundle_latest::ResourceBytes,
    HolochainVersion,
  },
  HolochainManager,
};
use lair_keystore_manager::utils::create_dir_if_necessary;
use serde::{Serialize, Deserialize};
use std::{
  collections::HashMap,
  fs::{self, File},
  path::{Path, PathBuf},
};

use crate::{
  error::LaunchWebAppManagerError,
  installed_web_app_info::{InstalledWebAppInfo, WebUiInfo},
  utils::unzip_file,
};

pub struct WebAppManager {
  environment_path: PathBuf,
  pub holochain_manager: HolochainManager,
  allocated_ports: HashMap<String, u16>,
}

impl WebAppManager {
  pub async fn launch(
    version: HolochainVersion,
    mut config: LaunchHolochainConfig,
    password: String,
  ) -> Result<Self, LaunchWebAppManagerError> {
    let environment_path = config.environment_path.clone();

    let conductor_data_path = conductor_dir(&environment_path);
    let apps_data_dir = apps_data_dir(&environment_path);

    config.environment_path = conductor_data_path.clone();

    create_dir_if_necessary(&environment_path)?;
    create_dir_if_necessary(&conductor_data_path)?;
    create_dir_if_necessary(&apps_data_dir)?;

    let holochain_manager = HolochainManager::launch(version, config, password)
      .await
      .map_err(|err| LaunchWebAppManagerError::LaunchHolochainError(err))?;

    // Fetch the running apps
    let mut manager = WebAppManager {
      holochain_manager,
      environment_path,
      allocated_ports: HashMap::new(),
    };
    manager
      .on_running_apps_changed()
      .await
      .map_err(|err| LaunchWebAppManagerError::Other(err))?;

    Ok(manager)
  }

  pub async fn install_web_app(
    &mut self,
    app_id: String,
    web_app_bundle: WebAppBundle,
    network_seed: Option<String>,
    membrane_proofs: HashMap<String, MembraneProof>,
    agent_pub_key: Option<AgentPubKey>,
  ) -> Result<(), String> {
    let app_bundle = web_app_bundle
      .happ_bundle()
      .await
      .or(Err("Failed to resolve hApp bundle"))?;

    let web_ui_zip_bytes = web_app_bundle
      .web_ui_zip_bytes()
      .await
      .or(Err("Failed to resolve Web UI"))?;

    // Install app UI in folder
    self.install_app_ui(app_id.clone(), web_ui_zip_bytes.to_vec())?;

    // Install app in conductor manager
    if let Err(err) = self
      .holochain_manager
      .install_app(
        app_id.clone(),
        app_bundle,
        network_seed,
        membrane_proofs,
        agent_pub_key,
      )
      .await
    {
      self.uninstall_app_ui(app_id)?;

      return Err(err);
    }

    self.on_running_apps_changed().await?;

    Ok(())
  }

  pub fn get_allocated_port(&self, app_id: &String) -> Option<u16> {
    self.allocated_ports.get(app_id).map(|u| u.clone())
  }

  fn install_app_ui(
    &mut self,
    app_id: String,
    web_ui_zip_bytes: ResourceBytes,
  ) -> Result<(), String> {
    let ui_folder_path = app_assets_dir(&self.environment_path, &app_id);
    let ui_zip_path = apps_data_dir(&self.environment_path).join(format!("{}.zip", app_id));

    fs::write(ui_zip_path.clone(), web_ui_zip_bytes).or(Err("Failed to write Web UI Zip file"))?;

    let file = File::open(ui_zip_path.clone()).or(Err("Failed to read Web UI Zip file"))?;
    unzip_file(file, ui_folder_path)?;

    fs::remove_file(ui_zip_path).or(Err("Failed to remove happ bundle"))?;

    Ok(())
  }

  /// Uninstalls the UI assets and tauri's localStorage associated to the given app
  fn uninstall_app_ui(&mut self, app_id: String) -> Result<(), String> {
    let ui_folder_path = app_assets_dir(&self.environment_path, &app_id);
    let local_storage_path = app_local_storage_dir(&self.environment_path, &app_id);

    if Path::new(&ui_folder_path).exists() {
      fs::remove_dir_all(ui_folder_path).or(Err("Failed to remove UI folder"))?;
    }

    if Path::new(&local_storage_path).exists() {
      fs::remove_dir_all(local_storage_path).or(Err("Failed to remove app's localStorage folder"))?;
    }

    Ok(())
  }

  async fn on_running_apps_changed(&mut self) -> Result<(), String> {
    let _installed_apps = self.list_apps().await?;
    Ok(())
  }

  fn get_web_ui_info(&self, app_id: String) -> Result<WebUiInfo, String> {
    let ui_folder_path = app_assets_dir(&self.environment_path, &app_id);

    match self.is_web_app(app_id.clone()) {
      true => Ok(WebUiInfo::WebApp {
        path_to_web_app: ui_folder_path,
        app_ui_port: self
          .allocated_ports
          .get(&app_id)
          .ok_or(format!(
            "This application was installed but we didn't allocate any port to it: {}",
            app_id,
          ))?
          .clone(),
      }),
      false => Ok(WebUiInfo::Headless),
    }
  }

  pub fn get_ui_index_path(&self, app_id: &String) -> PathBuf {
    app_assets_dir(&self.environment_path, &app_id).join("index.html")
  }

  pub fn get_app_ui_dir(&self, app_id: &String) -> PathBuf {
    app_assets_dir(&self.environment_path, &app_id)
  }

  pub fn get_app_local_storage_dir(&self, app_id: &String) -> PathBuf {
    app_local_storage_dir(&self.environment_path, &app_id)
  }


  pub fn kill(self) -> Result<(), String> {
    self.holochain_manager.kill()
  }

  pub async fn install_app(
    &mut self,
    app_id: String,
    app_bundle: AppBundle,
    network_seed: Option<String>,
    membrane_proofs: HashMap<String, MembraneProof>,
    agent_pub_key: Option<AgentPubKey>,
  ) -> Result<(), String> {
    // Install app in conductor manager
    self
      .holochain_manager
      .install_app(app_id, app_bundle, network_seed, membrane_proofs, agent_pub_key)
      .await
      .map_err(|err| {
        log::error!("Error installing hApp in the conductor: {}", err);
        err
      })?;

    self.on_running_apps_changed().await?;

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

    self.on_running_apps_changed().await?;

    Ok(())
  }

  pub async fn start_app(&mut self, app_id: String) -> Result<(), String> {
    self.holochain_manager.start_app(app_id.clone()).await?;

    self.on_running_apps_changed().await?;

    Ok(())
  }

  pub async fn enable_app(&mut self, app_id: String) -> Result<(), String> {
    self.holochain_manager.enable_app(app_id.clone()).await?;

    self.on_running_apps_changed().await?;

    Ok(())
  }

  pub async fn disable_app(&mut self, app_id: String) -> Result<(), String> {
    self.holochain_manager.disable_app(app_id.clone()).await?;

    self.on_running_apps_changed().await?;

    Ok(())
  }

  pub async fn list_apps(&mut self) -> Result<Vec<InstalledWebAppInfo>, String> {
    let installed_apps = self.holochain_manager.list_apps().await?;

    self.allocate_necessary_ports(&installed_apps);

    let installed_web_apps = installed_apps
      .into_iter()
      .map(|installed_app| {
        let web_ui_info = self.get_web_ui_info(installed_app.installed_app_id.clone())?;
        Ok(InstalledWebAppInfo {
          installed_app_info: installed_app,
          web_ui_info,
        })
      })
      .collect::<Result<Vec<InstalledWebAppInfo>, String>>()?;

    Ok(installed_web_apps)
  }

  fn is_web_app(&self, app_id: String) -> bool {
    let ui_folder_path = app_assets_dir(&self.environment_path, &app_id);

    Path::new(&ui_folder_path).exists()
  }

  fn allocate_necessary_ports(&mut self, installed_apps: &Vec<AppInfo>) -> () {
    let web_apps: Vec<AppInfo> = installed_apps
      .iter()
      .filter(|app| self.is_web_app(app.installed_app_id.clone()))
      .cloned()
      .collect();

    let mut installed_app_ids: HashMap<String, bool> = HashMap::new();

    // Allocate new ports for newly installed apps
    for web_app in web_apps {
      if !self.allocated_ports.contains_key(&web_app.installed_app_id) {
        let free_port = portpicker::pick_unused_port().expect("No ports free");
        self
          .allocated_ports
          .insert(web_app.installed_app_id.clone(), free_port);
      }

      installed_app_ids.insert(web_app.installed_app_id, true);
    }

    let allocated_app_ids: Vec<String> = self.allocated_ports.keys().cloned().collect();

    // Remove apps no longer installed
    for allocated_app_id in allocated_app_ids {
      if !installed_app_ids.contains_key(&allocated_app_id) {
        self.allocated_ports.remove(&allocated_app_id);
      }
    }
  }

  pub fn admin_interface_port(&self) -> u16 {
    self.holochain_manager.admin_interface_port()
  }

  pub fn app_interface_port(&mut self) -> u16 {
    self.holochain_manager.app_interface_port()
  }

  pub fn get_storage_info(&self) -> Result<StorageInfo, String> {
    let ui_path = apps_data_dir(&self.environment_path);
    let conductor_path = conductor_dir(&self.environment_path);
    let uis_size = fs_extra::dir::get_size(ui_path)
      .map_err(|e| format!("Failed to get UI directory size: {:?}", e))?;
    let authored_size = fs_extra::dir::get_size(conductor_path.join("authored"))
      .map_err(|e| format!("Failed to get conductor directory size: {:?}", e))?;
    let cache_size = fs_extra::dir::get_size(conductor_path.join("cache"))
      .map_err(|e| format!("Failed to get conductor directory size: {:?}", e))?;
    let conductor_size = fs_extra::dir::get_size(conductor_path.join("conductor"))
      .map_err(|e| format!("Failed to get conductor directory size: {:?}", e))?;
    let dht_size = fs_extra::dir::get_size(conductor_path.join("dht"))
      .map_err(|e| format!("Failed to get conductor directory size: {:?}", e))?;
    let p2p_size = fs_extra::dir::get_size(conductor_path.join("p2p"))
      .map_err(|e| format!("Failed to get conductor directory size: {:?}", e))?;
    let wasm_size = fs_extra::dir::get_size(conductor_path.join("wasm"))
      .map_err(|e| format!("Failed to get conductor directory size: {:?}", e))?;

    Ok(
      StorageInfo {
        uis: uis_size,
        authored: authored_size,
        cache: cache_size,
        conductor: conductor_size,
        dht: dht_size,
        p2p: p2p_size,
        wasm: wasm_size,
      }
    )

  }
}

/// Path to the apps folder relative to a root directory
/// (normally relative to the holochain version's "data directory")
fn apps_data_dir(root_path: &PathBuf) -> PathBuf {
  root_path.join("apps")
}

/// Path where UI assets of the given app are stored, relative
/// to a root directory (normally relative to the holochain version's "data directory")
fn app_assets_dir(root_path: &PathBuf, app_id: &String) -> PathBuf {
  apps_data_dir(root_path).join(app_id).join("assets")
}

/// Path where localStorage of the given app is stored, relative
/// to a root directory (normally relative to the holochain version's "data directory")
fn app_local_storage_dir(root_path: &PathBuf, app_id: &String) -> PathBuf {
  apps_data_dir(root_path).join(app_id).join("tauri")
}

/// Path where the conductor databases are stored, relative to a root
/// directory (normally relative to the holochain version's "data directory")
fn conductor_dir(root_path: &PathBuf) -> PathBuf {
  root_path.join("conductor")
}



#[derive(Serialize, Deserialize, Debug)]
pub struct StorageInfo {
  uis: u64,
  authored: u64,
  cache: u64,
  conductor: u64,
  dht: u64,
  p2p: u64,
  wasm: u64,
}