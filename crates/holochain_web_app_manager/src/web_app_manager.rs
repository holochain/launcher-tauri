use holochain_manager::{
  config::LaunchHolochainConfig,
  versions::{
    holochain_conductor_api_latest::InstalledAppInfo,
    holochain_types_latest::{
      prelude::{AppBundle, SerializedBytes},
      web_app::WebAppBundle,
    },
    launch_holochain,
    mr_bundle_latest::ResourceBytes,
    HolochainManagerEnum, HolochainVersion,
  },
  HolochainManager,
};
use lair_keystore_manager::utils::create_dir_if_necessary;
use std::{
  collections::HashMap,
  fs::{self, File},
  path::{Path, PathBuf},
};
use tauri::{AppHandle, Manager};

use crate::{
  caddy::manager::CaddyManager,
  error::LaunchWebAppManagerError,
  installed_web_app_info::{InstalledWebAppInfo, WebUiInfo},
  utils::unzip_file,
};

pub struct WebAppManager {
  environment_path: PathBuf,
  holochain_manager: HolochainManagerEnum,
  caddy_manager: CaddyManager,
  allocated_ports: HashMap<String, u16>,
  app_handle: AppHandle,
}

impl WebAppManager {
  pub async fn launch(
    version: HolochainVersion,
    config: LaunchHolochainConfig,
    password: String,
    app_handle: AppHandle,
  ) -> Result<Self, LaunchWebAppManagerError> {
    let environment_path = config.environment_path.clone();

    let conductor_data_path = environment_path.join("conductor");
    let ui_data_path = uis_data_path(&environment_path);

    let new_config = LaunchHolochainConfig {
      environment_path: conductor_data_path.clone(),
      ..config.clone()
    };

    create_dir_if_necessary(&environment_path)?;
    create_dir_if_necessary(&conductor_data_path)?;
    create_dir_if_necessary(&ui_data_path)?;

    let holochain_manager = launch_holochain(version, new_config, password)
      .await
      .map_err(|err| LaunchWebAppManagerError::LaunchHolochainError(err))?;

    let app_interface_port = holochain_manager.app_interface_port();

    let caddy_manager = CaddyManager::launch(
      environment_path.clone(),
      config.admin_port,
      app_interface_port,
    )
    .map_err(|err| LaunchWebAppManagerError::LaunchCaddyError(err))?;

    // Fetch the running apps and update caddyfile to already serve them
    let mut manager = WebAppManager {
      holochain_manager,
      environment_path,
      caddy_manager,
      allocated_ports: HashMap::new(),
      app_handle,
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
    uid: Option<String>,
    membrane_proofs: HashMap<String, SerializedBytes>,
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
      .install_app(app_id.clone(), app_bundle, uid, membrane_proofs)
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
    let ui_folder_path = app_ui_path(&self.environment_path, &app_id);
    let ui_zip_path = uis_data_path(&self.environment_path).join(format!("{}.zip", app_id));

    fs::write(ui_zip_path.clone(), web_ui_zip_bytes).or(Err("Failed to write Web UI Zip file"))?;

    let file = File::open(ui_zip_path.clone()).or(Err("Failed to read Web UI Zip file"))?;
    unzip_file(file, ui_folder_path)?;

    fs::remove_file(ui_zip_path).or(Err("Failed to remove happ bundle"))?;

    Ok(())
  }

  fn uninstall_app_ui(&mut self, app_id: String) -> Result<(), String> {
    let ui_folder_path = app_ui_path(&self.environment_path, &app_id);

    if Path::new(&ui_folder_path).exists() {
      fs::remove_dir_all(ui_folder_path).or(Err("Failed to remove UI folder"))?;
    }

    Ok(())
  }

  async fn on_running_apps_changed(&mut self) -> Result<(), String> {
    let installed_apps = self.list_apps().await?;

    self
      .caddy_manager
      .update_running_apps(&installed_apps)
      .map_err(|err| format!("Error reloading caddy {:?}", err))?;

    self
      .app_handle
      .emit_all("running_apps_changed", ())
      .map_err(|err| format!("Error sending running_apps_changed event {:?}", err))?;

    Ok(())
  }

  fn get_web_ui_info(&self, app_id: String) -> Result<WebUiInfo, String> {
    let ui_folder_path = app_ui_path(&self.environment_path, &app_id);

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

  pub fn kill(self) -> Result<(), String> {
    self.holochain_manager.kill()?;
    self.caddy_manager.kill()
  }

  pub async fn install_app(
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
    let ui_folder_path = app_ui_path(&self.environment_path, &app_id);

    Path::new(&ui_folder_path).exists()
  }

  fn allocate_necessary_ports(&mut self, installed_apps: &Vec<InstalledAppInfo>) -> () {
    let web_apps: Vec<InstalledAppInfo> = installed_apps
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
}

fn uis_data_path(root_path: &PathBuf) -> PathBuf {
  root_path.join("uis")
}

fn app_ui_path(root_path: &PathBuf, app_id: &String) -> PathBuf {
  uis_data_path(root_path).join(app_id)
}
