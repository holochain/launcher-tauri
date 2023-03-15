use futures::lock::Mutex;
use holochain_manager::{
  config::LaunchHolochainConfig,
  versions::{
    holochain_conductor_api_latest::AppInfo,
    holochain_types_latest::{
      prelude::{AgentPubKey, AppBundle, CellId, MembraneProof},
      web_app::WebAppBundle,
    },
    mr_bundle_latest::ResourceBytes,
    HolochainVersion,
  },
  HolochainManager,
};
use lair_keystore_manager::utils::create_dir_if_necessary;
use serde::{Deserialize, Serialize};
use std::{
  collections::HashMap,
  fs::{self, File},
  path::{Path, PathBuf},
  sync::Arc,
};
use tauri::{AppHandle, Manager};

use crate::{
  error::LaunchWebAppManagerError,
  installed_web_app_info::{InstalledWebAppInfo, WebUiInfo},
  utils::unzip_file,
};

pub struct WebAppManager {
  environment_path: PathBuf,
  pub holochain_manager: HolochainManager,
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
    happ_release_hash: Option<String>,
    gui_release_hash: Option<String>,
  ) -> Result<AppInfo, String> {
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

    // println!("Installing webhapp. Hashes: {:?}, {:?}", happ_release_hash, gui_release_hash);

    // Try to write hashes first so if that fails, don't even install the app
    match (happ_release_hash, gui_release_hash.clone()) {
      (Some(h_hash), Some(_g_hash)) => {
        self.store_happ_release_hash(h_hash, &app_id)
          .map_err(|e| format!("Failed to store happ release hash to .happrelease file: {:?}", e))?;
        // gui release hash will be stored during install_app_ui
      },
      (None, None) => (),
      _ => return Err(String::from("Got only one of gui_release_hash or happ_release_hash. Pass either none of them if installing a .webhapp from filesystem or both if installing a .webhapp from the App Library."))
    }

    // Install app UI in folder
    self.install_app_ui(
      app_id.clone(),
      web_ui_zip_bytes.into_owned(),
      &default_ui_name,
      gui_release_hash,
    )?;

    // Install app in conductor manager
    let install_result = self
      .holochain_manager
      .install_app(
        app_id.clone(),
        app_bundle,
        network_seed,
        membrane_proofs,
        agent_pub_key,
      )
      .await;
    match install_result {
      Ok(app_info) => {
        self.on_running_apps_changed().await?;
        Ok(app_info)
      }
      Err(err) => {
        self.uninstall_app_data(app_id)?;

        Err(err)
      }
    }
  }

  fn install_app_ui(
    &mut self,
    app_id: String,
    web_ui_zip_bytes: ResourceBytes,
    ui_name: &String,
    gui_release_hash: Option<String>,
  ) -> Result<(), String> {
    if let Some(hash) = gui_release_hash {
      self.store_gui_release_hash(hash, &app_id, &ui_name)?;
    }

    // Careful! The ui_folder_path here needs to be the same as the one being deleted in update_app_ui()
    // in case of a failed installation. Otherwise wrong stuff may be deleted.
    let ui_folder_path = app_assets_dir(&self.environment_path, &app_id, ui_name);
    let ui_zip_path = apps_data_dir(&self.environment_path).join(format!("{}.zip", app_id));

    fs::write(ui_zip_path.clone(), web_ui_zip_bytes.into_inner())
      .or(Err("Failed to write Web UI Zip file"))?;

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
    gui_release_hash: Option<String>,
  ) -> Result<(), String> {
    // move folder of previous assets to a temporary backup folder in case installation fails
    let ui_folder_path = app_assets_dir(&self.environment_path, &app_id, ui_name);
    let temp_old_ui_path =
      app_ui_dir(&self.environment_path, &app_id, ui_name).join("assets_temp_backup");
    fs::rename(ui_folder_path.clone(), temp_old_ui_path.clone()).map_err(|e| {
      format!(
        "Failed to move currently installed UI assets to temporary backup location: {:?}",
        e
      )
    })?;

    if gui_release_hash == None {
      log::warn!("WARNING: App UI updated without passing a gui release hash. This only expected if a GUI is updated from the filesystem instead of through fetching it form the DevHub");
    }

    // write zip file with new UI assets to disk in order to unpack it
    match self.install_app_ui(app_id, web_ui_zip_bytes, ui_name, gui_release_hash) {
      Ok(()) => (),
      Err(e) => {
        log::error!("Failed to install app ui during update_app_ui: {:?}", e);
        // restore previous state
        fs::remove_dir_all(ui_folder_path.clone())
          .map_err(|e| format!("Failed to remove assets dir when trying to restore the pre-update state due to failed installation of the new app UI: {:?}", e))?;
        fs::rename(temp_old_ui_path, ui_folder_path)
          .map_err(|e| format!("Failed to rename temporary assets backup dir when trying to restore the pre-update state due to failed installation of the new app UI: {:?}", e))?;
        return Err(e);
      }
    }

    // If installation was successful, remove the temporary backup directory
    fs::remove_dir_all(temp_old_ui_path).map_err(|e| {
      format!(
        "Failed to remove temporary backup folder for assets after successful installation: {:?}",
        e
      )
    })?;

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
      fs::remove_dir_all(local_storage_path)
        .or(Err("Failed to remove app's localStorage folder"))?;
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

    let gui_release_hash = self.get_gui_release_hash(&app_id, ui_name);

    match self.is_web_app(app_id.clone()) {
      true => Ok(WebUiInfo::WebApp {
        path_to_ui: ui_folder_path,
        gui_release_hash,
      }),
      false => Ok(WebUiInfo::Headless),
    }
  }

  fn is_web_app(&self, app_id: String) -> bool {
    // Assuming only one single default UI per app at the moment.
    let default_ui_name = String::from("default");
    let ui_folder_path = app_assets_dir(&self.environment_path, &app_id, &default_ui_name);

    Path::new(&ui_folder_path).exists()
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
    happ_release_hash: Option<String>,
  ) -> Result<(), String> {
    // Try to write hashes first so if that fails, don't even install the app
    // Note: a hApp release hash will only be passed if the hApp is installed
    // from the AppLibrary
    if let Some(hash) = happ_release_hash {
      self.store_happ_release_hash(hash, &app_id)?;
    }

    // Install app in conductor manager
    self
      .holochain_manager
      .install_app(
        app_id,
        app_bundle,
        network_seed,
        membrane_proofs,
        agent_pub_key,
      )
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

    // Assuming only one single default UI per app at the moment.
    let default_ui_name = String::from("default");

    let installed_web_apps = installed_apps
      .into_iter()
      .map(|installed_app| {
        let web_ui_info =
          self.get_web_ui_info(installed_app.installed_app_id.clone(), &default_ui_name)?;
        // Currently only 1 UI supported
        let mut web_uis = HashMap::new();
        web_uis.insert(default_ui_name.clone(), web_ui_info);

        let happ_release_hash = self.get_happ_release_hash(&installed_app.installed_app_id);

        Ok(InstalledWebAppInfo {
          installed_app_info: installed_app,
          happ_release_hash,
          web_uis,
        })
      })
      .collect::<Result<Vec<InstalledWebAppInfo>, String>>()?;

    Ok(installed_web_apps)
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

    Ok(StorageInfo {
      uis: uis_size,
      authored: authored_size,
      cache: cache_size,
      conductor: conductor_size,
      dht: dht_size,
      p2p: p2p_size,
      wasm: wasm_size,
    })
  }

  /// Stores the hash of a happ release of the given hApp to the filesystem
  /// EntryHash must be passed as a base64 string
  pub fn store_happ_release_hash(&self, hash: String, app_id: &String) -> Result<(), String> {
    let app_data_dir = app_data_dir(&self.environment_path, app_id);

    create_dir_if_necessary(&app_data_dir).map_err(|e| {
      format!(
        "Failed to create app's data directory before storing happ release hash: {:?}",
        e
      )
    })?;

    let dot_happrelease_path = app_data_dir.join(".happrelease");
    // if there is already a .happrelease file, store its contents to a .happrelease.previous file in order to be able
    // to revert upgrades if necessary
    if dot_happrelease_path.exists() {
      let dot_happrelease_dot_previous_path = app_data_dir.join(".happrelease.previous");
      std::fs::rename(
        dot_happrelease_path.clone(),
        dot_happrelease_dot_previous_path,
      )
      .map_err(|e| {
        format!(
          "Failed to rename .happrelease file to .happrelease.previous file: {:?}",
          e
        )
      })?;
    }

    // println!("Storing happ release hash to the following path: {:?}", dot_happrelease_path);

    std::fs::write(dot_happrelease_path, hash).map_err(|e| {
      format!(
        "Failed to write happ release hash to .happrelease file: {:?}",
        e
      )
    })
  }

  /// Stores the hash of a gui release of the given hApp to the filesystem
  /// EntryHash must be passed as a base64 string
  pub fn store_gui_release_hash(
    &self,
    hash: String,
    app_id: &String,
    ui_name: &String,
  ) -> Result<(), String> {
    let app_gui_dir = app_ui_dir(&self.environment_path, app_id, ui_name);
    create_dir_if_necessary(&app_gui_dir).map_err(|e| {
      format!(
        "Failed to create app's data directory before storing happ release hash: {:?}",
        e
      )
    })?;

    let dot_guirelease_path = app_gui_dir.join(".guirelease");
    // if there is already a .guirelease file, store its contents to a .guirelease.previous file in order to be able
    // to revert upgrades if necessary
    if dot_guirelease_path.exists() {
      let dot_guirelease_dot_previous_path =
        app_data_dir(&self.environment_path, app_id).join(".guirelease.previous");

      std::fs::rename(
        dot_guirelease_path.clone(),
        dot_guirelease_dot_previous_path,
      )
      .map_err(|e| {
        format!(
          "Failed to rename .guirelease file to .guirelease.previous file: {:?}",
          e
        )
      })?;
    }

    // println!("Storing GUI release hash to the following path: {:?}", dot_guirelease_path);

    std::fs::write(dot_guirelease_path, hash).map_err(|e| {
      format!(
        "Failed to write GUI release hash to .guirelease file: {:?}",
        e
      )
    })
  }

  /// Reads the happ release hash of an app
  pub fn get_happ_release_hash(&self, app_id: &String) -> Option<String> {
    match fs::read_to_string(app_data_dir(&self.environment_path, app_id).join(".happrelease")) {
      Ok(s) => Some(s),
      Err(_) => None,
    }
  }

  /// Reads the gui release hash of an app UI
  pub fn get_gui_release_hash(&self, app_id: &String, ui_name: &String) -> Option<String> {
    match fs::read_to_string(
      app_ui_dir(&self.environment_path, app_id, ui_name).join(".guirelease"),
    ) {
      Ok(s) => Some(s),
      Err(_) => None,
    }
  }
}

/// Derives the window label from the app id and the holochain id
/// The window label will be of the format [holochain version]#[app id with some special characters removed]
pub fn derive_window_label(app_id: &String) -> String {
  // !! it is important to have the window label not be uniquely defined by the app id to ensure
  // it's possible to unambiguously differentiate this window from the admin window !!
  let mut window_label = app_id
    .clone()
    .replace("-", "--")
    .replace(" ", "-")
    .replace(".", "_");
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
  apps_data_dir(root_path)
    .join(app_id)
    .join("uis")
    .join(ui_name)
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
