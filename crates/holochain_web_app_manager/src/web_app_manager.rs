use holochain_manager::{
  config::LaunchHolochainConfig,
  versions::{
    holochain_conductor_api_latest::AppInfo,
    holochain_types_latest::{
      prelude::{AgentPubKey, AppBundle, MembraneProof, CellId, DnaHash, AnyDhtHash, DnaHashB64, AnyDhtHashB64},
      web_app::WebAppBundle,
    },
    mr_bundle_latest::ResourceBytes,
    HolochainVersion,
  },
  HolochainManager,
};
use lair_keystore_manager::utils::create_dir_if_necessary;
use serde::{Serialize, Deserialize};
use futures::lock::Mutex;
use tauri::{AppHandle, Manager};
use std::{
  collections::HashMap,
  fs::{self, File},
  path::{Path, PathBuf}, sync::Arc,
};

use crate::{
  error::LaunchWebAppManagerError,
  installed_web_app_info::{InstalledWebAppInfo, WebUiInfo},
  utils::unzip_file,
};



//// NOTE: This is not necessarily an HRL. For example UI's stored on the
/// DevHub need to be accessed via the `happs` cell despite being actually
/// stored in the `web_assets` cell. The DevHub is making a bridge call
/// internally.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResourceLocator {
    pub dna_hash: DnaHash,
    pub resource_hash: AnyDhtHash,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ResourceLocatorB64 {
    pub dna_hash: DnaHashB64,
    pub resource_hash: AnyDhtHashB64,
}

impl Into<ResourceLocator> for ResourceLocatorB64 {
  fn into(self) -> ResourceLocator {
    ResourceLocator {
      dna_hash: DnaHash::from(self.dna_hash),
      resource_hash: AnyDhtHash::from(self.resource_hash),
    }
  }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ReleaseInfo {
  pub resource_locator: Option<ResourceLocatorB64>,
  pub version: Option<String>,
}


pub struct WebAppManager {
  environment_path: PathBuf,
  app_handle: Arc<AppHandle>,
  pub holochain_manager: HolochainManager,
  allocated_ports: HashMap<String, u16>,
}

impl WebAppManager {
  pub async fn launch(
    version: HolochainVersion,
    mut config: LaunchHolochainConfig,
    app_handle: Arc<AppHandle>,
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
      app_handle,
      environment_path,
      allocated_ports: HashMap::new(),
    };
    manager
      .on_running_apps_changed()
      .await
      .map_err(|err| LaunchWebAppManagerError::Other(err))?;

    Ok(manager)
  }


  /// Install happ with UI
  pub async fn install_web_app(
    &mut self,
    app_id: String,
    web_app_bundle: WebAppBundle,
    network_seed: Option<String>,
    membrane_proofs: HashMap<String, MembraneProof>,
    agent_pub_key: Option<AgentPubKey>,
    happ_release_info: Option<ReleaseInfo>,
    gui_release_info: Option<ReleaseInfo>,
  ) -> Result<(), String> {
    let app_bundle = web_app_bundle
      .happ_bundle()
      .await
      .or(Err("Failed to resolve hApp bundle"))?;

    let web_ui_zip_bytes = web_app_bundle
      .web_ui_zip_bytes()
      .await
      .or(Err("Failed to resolve Web UI"))?;


    // Assuming only one single default UI per app at the moment.
    let default_ui_name = String::from("default");

    // Try to write hashes first so if that fails, don't even install the app
    match (happ_release_info, gui_release_info.clone()) {
      (Some(h_info), Some(_g_info)) => {
        self.store_happ_release_info(h_info, &app_id)
          .map_err(|e| format!("Failed to store happ release hash to .happrelease file: {:?}", e))?;
        // gui release hash will be stored during install_app_ui
      },
      (None, None) => (),
      _ => return Err(String::from("Got only one of happ_release_info or gui_release_info. Pass either none of them if installing a .webhapp from filesystem or both if installing a .webhapp from the App Library."))
    }

    // Install app UI in folder
    self.install_app_ui(app_id.clone(), web_ui_zip_bytes.into_owned(), &default_ui_name, gui_release_info)?;

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
      self.uninstall_app_data(app_id)?;

      return Err(err);
    }

    self.on_running_apps_changed().await?;

    Ok(())
  }

  pub fn get_allocated_port(&self, app_id: &String) -> Option<u16> {
    self.allocated_ports.get(app_id).map(|u| u.clone())
  }

  pub fn install_app_ui(
    &mut self,
    app_id: String,
    web_ui_zip_bytes: ResourceBytes,
    ui_name: &String,
    gui_release_info: Option<ReleaseInfo>,
  ) -> Result<(), String> {

    if let Some(info) = gui_release_info {
      self.store_gui_release_info(info, &app_id, &ui_name)?;
    }

    // Careful! The ui_folder_path here needs to be the same as the one being deleted in update_app_ui()
    // in case of a failed installation. Otherwise wrong stuff may be deleted.
    let ui_folder_path = app_assets_dir(&self.environment_path, &app_id, ui_name);
    let ui_zip_path = apps_data_dir(&self.environment_path).join(format!("{}.zip", app_id));

    fs::write(ui_zip_path.clone(), web_ui_zip_bytes.into_inner()).or(Err("Failed to write Web UI Zip file"))?;

    let file = File::open(ui_zip_path.clone()).or(Err("Failed to read Web UI Zip file"))?;
    unzip_file(file, ui_folder_path)?;

    fs::remove_file(ui_zip_path).or(Err("Failed to remove happ bundle"))?;

    Ok(())
  }

  pub fn update_app_ui(
    &mut self,
    app_id: String,
    web_ui_zip_bytes: ResourceBytes,
    ui_name: &String,
    gui_release_info: Option<ReleaseInfo>,
  ) -> Result<(), String> {
    // move folder of previous assets to a temporary backup folder in case installation fails
    let ui_folder_path = app_assets_dir(&self.environment_path, &app_id, ui_name);
    let temp_old_ui_path = app_ui_dir(&self.environment_path, &app_id, ui_name).join("assets_temp_backup");
    fs::rename(ui_folder_path.clone(), temp_old_ui_path.clone())
      .map_err(|e| format!("Failed to move currently installed UI assets to temporary backup location: {:?}", e))?;

    if gui_release_info.is_none() {
      log::warn!("WARNING: App UI updated without passing a gui release hash. This only expected if a GUI is updated from the filesystem instead of through fetching it form the DevHub");
    }

    // write zip file with new UI assets to disk in order to unpack it
    match self.install_app_ui(app_id, web_ui_zip_bytes, ui_name, gui_release_info) {
      Ok(()) => (),
      Err(e) => {
        log::error!("Failed to install app ui during update_app_ui: {:?}", e);
        // restore previous state
        fs::remove_dir_all(ui_folder_path.clone())
          .map_err(|e| format!("Failed to remove assets dir when trying to restore the pre-update state due to failed installation of the new app UI: {:?}", e))?;
        fs::rename(temp_old_ui_path, ui_folder_path)
          .map_err(|e| format!("Failed to rename temporary assets backup dir when trying to restore the pre-update state due to failed installation of the new app UI: {:?}", e))?;
        return Err(e)
      }
    }

    // If installation was successful, remove the temporary backup directory
    fs::remove_dir_all(temp_old_ui_path)
      .map_err(|e| format!("Failed to remove temporary backup folder for assets after successful installation: {:?}", e))?;

    Ok(())
  }

  /// Uninstalls the UI assets and tauri's localStorage associated to the given app
  fn _uninstall_app_ui(&mut self, app_id: String, ui_name: &String) -> Result<(), String> {

    let ui_folder_path = app_assets_dir(&self.environment_path, &app_id, ui_name);
    let local_storage_path = app_local_storage_dir(&self.environment_path, &app_id, ui_name);

    if Path::new(&ui_folder_path).exists() {
      fs::remove_dir_all(ui_folder_path).or(Err("Failed to remove UI folder"))?;
    }

    if Path::new(&local_storage_path).exists() {
      fs::remove_dir_all(local_storage_path).or(Err("Failed to remove app's localStorage folder"))?;
    }

    Ok(())
  }


  /// Uninstalls the data of all UI's of this app as well as tauri's localStorage associated to those UI's
  fn uninstall_app_data(&mut self, app_id: String) -> Result<(), String> {

    let ui_folder_path = app_data_dir(&self.environment_path, &app_id);

    if Path::new(&ui_folder_path).exists() {
      fs::remove_dir_all(ui_folder_path).or(Err("Failed to remove app's data dir"))?;
    }

    Ok(())
  }


  async fn on_running_apps_changed(&mut self) -> Result<(), String> {
    let _installed_apps = self.list_apps().await?;
    Ok(())
  }

  fn get_web_ui_info(&self, app_id: String, ui_name: &String) -> Result<WebUiInfo, String> {
    let ui_folder_path = app_assets_dir(&self.environment_path, &app_id, ui_name);

    let gui_release_info = self.get_gui_release_info(&app_id, ui_name);

    match self.is_web_app(app_id.clone()) {
      true => Ok(WebUiInfo::WebApp {
        path_to_ui: ui_folder_path,
        app_ui_port: self
          .allocated_ports
          .get(&app_id)
          .ok_or(format!(
            "This application was installed but we didn't allocate any port to it: {}",
            app_id,
          ))?
          .clone(),
          gui_release_info,
      }),
      false => Ok(WebUiInfo::Headless),
    }
  }

  pub fn get_app_assets_dir(&self, app_id: &String, ui_name: &String) -> PathBuf {
    app_assets_dir(&self.environment_path, &app_id, ui_name)
  }

  pub fn get_app_local_storage_dir(&self, app_id: &String, ui_name: &String) -> PathBuf {
    app_local_storage_dir(&self.environment_path, &app_id, ui_name)
  }


  pub fn kill(self) -> Result<(), String> {
    self.holochain_manager.kill()
  }


  /// Install a happ *without* UI
  pub async fn install_app(
    &mut self,
    app_id: String,
    app_bundle: AppBundle,
    network_seed: Option<String>,
    membrane_proofs: HashMap<String, MembraneProof>,
    agent_pub_key: Option<AgentPubKey>,
    happ_release_info: Option<ReleaseInfo>,
  ) -> Result<(), String> {

    // Try to write hashes first so if that fails, don't even install the app
    // Note: a hApp release hash will only be passed if the hApp is installed
    // from the AppLibrary
    if let Some(info) = happ_release_info {
      self.store_happ_release_info(info, &app_id)?;
    }

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

  /// This uninstalls the happ from the conductor as well as all UI's and
  /// localStorage related to that happ from the filesystem
  pub async fn uninstall_app(&mut self, app_id: String) -> Result<(), String> {
    self
      .holochain_manager
      .uninstall_app(app_id.clone())
      .await
      .map_err(|err| {
        log::error!("Error uninstalling hApp in the conductor: {}", err);
        err
      })?;

    self.uninstall_app_data(app_id)?;

    self.on_running_apps_changed().await?;

    Ok(())
  }

  // pub async fn start_app(&mut self, app_id: String) -> Result<(), String> {
  //   self.holochain_manager.start_app(app_id.clone()).await?;

  //   self.on_running_apps_changed().await?;

  //   Ok(())
  // }

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

  pub async fn delete_clone(&mut self, app_id: String, cell_id: CellId) -> Result<(), String> {
    self.holochain_manager.delete_clone(app_id, cell_id).await?;

    self.on_running_apps_changed().await?;

    Ok(())
  }

  pub async fn list_apps(&mut self) -> Result<Vec<InstalledWebAppInfo>, String> {
    let installed_apps = self.holochain_manager.list_apps().await?;

    let mut updated_pubkey_map: HashMap<String, AgentPubKey> = HashMap::new();
    // update agent public key to tauri window label mapping
    for app_info in installed_apps.clone() {
      let window_label = derive_window_label(&app_info.installed_app_id);
      updated_pubkey_map.insert(window_label, app_info.agent_pub_key);
    }

    *self.app_handle.state::<Arc<Mutex<HashMap<String, AgentPubKey>>>>().lock().await = updated_pubkey_map;

    self.allocate_necessary_ports(&installed_apps);

    // Assuming only one single default UI per app at the moment.
    let default_ui_name = String::from("default");

    let installed_web_apps = installed_apps
      .into_iter()
      .map(|installed_app| {
        let web_ui_info = self.get_web_ui_info(installed_app.installed_app_id.clone(), &default_ui_name)?;
        // Currently only 1 UI supported
        let mut web_uis = HashMap::new();
        web_uis.insert(default_ui_name.clone(), web_ui_info);

        let happ_release_info = self.get_happ_release_info(&installed_app.installed_app_id);

        let icon_src = self.get_app_icon_src(&installed_app.installed_app_id);

        Ok(InstalledWebAppInfo {
          installed_app_info: installed_app,
          happ_release_info,
          web_uis,
          icon_src,
        })
      })
      .collect::<Result<Vec<InstalledWebAppInfo>, String>>()?;

    Ok(installed_web_apps)
  }

  fn is_web_app(&self, app_id: String) -> bool {
    // Assuming only one single default UI per app at the moment.
    let default_ui_name = String::from("default");
    let ui_folder_path = app_assets_dir(&self.environment_path, &app_id, &default_ui_name);

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


  /// Stores the hash of a happ release of the given hApp to the filesystem in a yaml file
  /// called .happrelease
  /// ActionHash must be passed as a base64 string
  pub fn store_happ_release_info(&self, info: ReleaseInfo, app_id: &String) -> Result<(), String> {

    let app_data_dir = app_data_dir(&self.environment_path, app_id);

    create_dir_if_necessary(&app_data_dir)
      .map_err(|e| format!("Failed to create app's data directory before storing happ release hash: {:?}", e))?;

    let dot_happrelease_path = app_data_dir.join(".happrelease");
    // if there is already a .happrelease file, store its contents to a .happrelease.previous file in order to be able
    // to revert upgrades if necessary
    if dot_happrelease_path.exists() {
      let dot_happrelease_dot_previous_path = app_data_dir.join(".happrelease.previous");
      std::fs::rename(dot_happrelease_path.clone(), dot_happrelease_dot_previous_path)
        .map_err(|e| format!("Failed to rename .happrelease file to .happrelease.previous file: {:?}", e))?;
    }

    // println!("Storing happ release hash to the following path: {:?}", dot_happrelease_path);
    let info_value = serde_yaml::to_value(info)
      .map_err(|e| format!("Failed to convert info of happ release to serde_yaml Value: {}", e))?;

    let info_string = serde_yaml::to_string(&info_value)
      .map_err(|e| format!("Failed to convert info of happ release from serde_yaml Value to string: {}", e))?;

    std::fs::write(dot_happrelease_path, info_string)
      .map_err(|e| format!("Failed to write happ release info to .happrelease file: {:?}", e))
  }

  /// Stores the hash of a gui release of the given hApp to the filesystem
  /// ActionHash must be passed as a base64 string
  pub fn store_gui_release_info(&self, info: ReleaseInfo, app_id: &String, ui_name: &String) -> Result<(), String> {

    let app_gui_dir = app_ui_dir(&self.environment_path, app_id, ui_name);
    create_dir_if_necessary(&app_gui_dir)
      .map_err(|e| format!("Failed to create app's data directory before storing gui release hash: {:?}", e))?;

    let dot_guirelease_path = app_gui_dir.join(".guirelease");
    // if there is already a .guirelease file, store its contents to a .guirelease.previous file in order to be able
    // to revert upgrades if necessary
    if dot_guirelease_path.exists() {
      let dot_guirelease_dot_previous_path = app_gui_dir.join(".guirelease.previous");

      std::fs::rename(dot_guirelease_path.clone(), dot_guirelease_dot_previous_path)
        .map_err(|e| format!("Failed to rename .guirelease file to .guirelease.previous file: {:?}", e))?;
    }

    // println!("Storing GUI release hash to the following path: {:?}", dot_guirelease_path);
    let info_value = serde_yaml::to_value(info)
      .map_err(|e| format!("Failed to convert ResourceLocator of GUI release info to serde_yaml Value: {}", e))?;

    let info_string = serde_yaml::to_string(&info_value)
      .map_err(|e| format!("Failed to convert info of GUI release from serde_yaml Value to string: {}", e))?;

    std::fs::write(dot_guirelease_path, info_string)
      .map_err(|e| format!("Failed to write GUI release info to .guirelease file: {:?}", e))
  }

  /// Reads the happ release hash of an app
  pub fn get_happ_release_info(&self, app_id: &String) -> Option<ReleaseInfo> {
    match fs::read_to_string(app_data_dir(&self.environment_path, app_id).join(".happrelease")) {
      Ok(s) => {
        let happ_release_info = serde_yaml::from_str::<ReleaseInfo>(s.as_str());
        match happ_release_info {
          Ok(info) => Some(info),
          Err(e) => {
            log::error!("Failed to read happ release info from .happrelease file: {}", e);
            None
          }
        }
      },
      Err(_) => None,
    }
  }

  /// Reads the gui release hash of an app UI
  pub fn get_gui_release_info(&self, app_id: &String, ui_name: &String) -> Option<ReleaseInfo> {
    match fs::read_to_string(app_ui_dir(&self.environment_path, app_id, ui_name).join(".guirelease")) {
      Ok(s) => {
        let gui_release_info = serde_yaml::from_str::<ReleaseInfo>(s.as_str());
        match gui_release_info {
          Ok(info) => Some(info),
          Err(e) => {
            log::error!("Failed to read GUI release ResourceLocator from .guirelease file: {}", e);
            None
          }
        }
      },
      Err(_) => None,
    }
  }

  /// Stores the app icon src
  /// The icon is expected to be a base64 string of the format 'data:image/png;base64,[...blabla...]'
  pub fn store_app_icon_src(&self, icon_src: String, app_id: &String) -> Result<(), String> {

    let app_data_dir = app_data_dir(&self.environment_path, app_id);

    create_dir_if_necessary(&app_data_dir)
      .map_err(|e| format!("Failed to create app's data directory before storing app icon src: {:?}", e))?;

    let icon_path = app_data_dir.join(".icon");

    std::fs::write(icon_path, icon_src)
      .map_err(|e| format!("Failed to write icon src to .icon file: {:?}", e))
  }

  /// Reads the app icon src
  /// The icon is expected to be a base64 string of the format 'data:image/png;base64,[...blabla...]'
  pub fn get_app_icon_src(&self, app_id: &String) -> Option<String> {
    match fs::read_to_string(app_data_dir(&self.environment_path, app_id).join(".icon")) {
      Ok(s) => Some(s),
      Err(_) => {
        log::error!("Failed to load icon src for app with id {}", app_id);
        None
      },
    }
  }


  pub async fn dump_network_stats(&mut self) -> Result<String, String> {
    self.holochain_manager.dump_network_stats().await
  }


}


/// Derives the window label from the app id and the holochain id
/// The window label will be of the format [holochain version]#[app id with some special characters removed]
pub fn derive_window_label(app_id: &String) -> String {
  // !! it is important to have the window label not be uniquely defined by the app id to ensure
  // it's possible to unambiguously differentiate this window from the admin window !!
  let mut window_label = app_id.clone().replace("-", "--").replace(" ", "-").replace(".", "_");
  window_label.push_str("--EXTERNAL");
  window_label
}

/// Path to the apps folder relative to a root directory
/// (normally relative to the holochain version's "data directory")
fn apps_data_dir(root_path: &PathBuf) -> PathBuf {
  root_path.join("apps")
}


/// Path to the apps folder relative to a root directory
/// (normally relative to the holochain version's "data directory")
fn app_data_dir(root_path: &PathBuf, app_id: &String) -> PathBuf {
  apps_data_dir(root_path).join(app_id)
}

/// Path where things related to a specific UI of the given app are stored, relative
/// to a root directory (normally relative to the holochain version's "data directory")
fn app_ui_dir(root_path: &PathBuf, app_id: &String, ui_name: &String) -> PathBuf {
  apps_data_dir(root_path).join(app_id).join("uis").join(ui_name)
}

/// Path where UI assets of the given app are stored, relative
/// to a root directory (normally relative to the holochain version's "data directory")
fn app_assets_dir(root_path: &PathBuf, app_id: &String, ui_name: &String) -> PathBuf {
  app_ui_dir(root_path, app_id, ui_name).join("assets")
}


/// Path where localStorage of the given app UI is stored, relative
/// to a root directory (normally relative to the holochain version's "data directory")
/// ui_name is the name of the UI assuming that there may be more than one UI per app
fn app_local_storage_dir(root_path: &PathBuf, app_id: &String, ui_name: &String) -> PathBuf {
  app_ui_dir(root_path, app_id, ui_name).join("tauri")
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