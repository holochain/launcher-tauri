use std::collections::HashMap;
use std::env::temp_dir;
use std::path::Path;
use std::time::SystemTime;
use std::{fs, time::Duration};

// NEW_VERSION change holochain_types version
use holochain_client::{AdminWebsocket, AgentPubKey, AppInfo, InstallAppPayload};
use holochain_types_0_2_2::prelude::{
  AppBundleSource, CellId, CloneCellId, DisableCloneCellPayload,
};
use lair_keystore_manager::utils::create_dir_if_necessary;
use tauri::api::process::CommandChild;

use crate::versions::holochain_types_latest::prelude::{AppBundle, MembraneProof};

use crate::{
  config::LaunchHolochainConfig,
  errors::LaunchHolochainError,
  launch::launch_holochain_process,
  versions::{version_manager::VersionManager, HolochainVersion},
};

pub struct HolochainManager {
  pub version: HolochainVersion,

  admin_interface_port: u16,
  app_interface_port: u16,

  // With this we are assuming that the Admin API won't change in the near future
  // If it changes, move this property down in the HolochainVersion
  ws: AdminWebsocket,

  command_child: CommandChild,
}

impl HolochainManager {
  pub async fn launch(
    version: HolochainVersion,
    config: LaunchHolochainConfig,
    password: String,
  ) -> Result<Self, LaunchHolochainError> {
    let conductor_config_path = config.conductor_config_dir.join("conductor-config.yaml");
    create_dir_if_necessary(&config.conductor_config_dir)?;
    create_dir_if_necessary(&config.environment_path)?;

    let version_manager = version.manager();

    let new_conductor_config = match Path::new(&conductor_config_path).exists() {
      true => {
        let current_config_str = fs::read_to_string(conductor_config_path.clone())?;

        version_manager
          .overwrite_config(
            current_config_str,
            config.admin_port,
            config.keystore_connection_url.clone(),
            config.bootstrap_server_url,
            config.signaling_server_url,
          )
          .map_err(|e| LaunchHolochainError::FailedToOverwriteConfig(e))?
      }
      false => version_manager.initial_config(
        config.admin_port,
        config.environment_path.clone(),
        config.keystore_connection_url.clone(),
        config.bootstrap_server_url,
        config.signaling_server_url,
      ),
    };

    fs::write(conductor_config_path.clone(), new_conductor_config.clone())
      .expect("Could not write conductor config");

    let command_child = launch_holochain_process(
      config.log_level,
      version,
      config.command,
      conductor_config_path,
      password,
    )
    .await?;

    std::thread::sleep(Duration::from_millis(100));

    // Try to connect twice. This fixes the os(111) error for now that occurs when the conducor is not ready yet.
    let mut ws = match AdminWebsocket::connect(format!("ws://localhost:{}", config.admin_port))
      .await
    {
      Ok(ws) => ws,
      Err(_) => {
        log::error!("[HOLOCHAIN {}] Could not connect to the AdminWebsocket. Starting another attempt in 5 seconds.", version);
        std::thread::sleep(Duration::from_millis(5000));
        AdminWebsocket::connect(format!("ws://localhost:{}", config.admin_port))
          .await
          .map_err(|err| LaunchHolochainError::CouldNotConnectToConductor(format!("{}", err)))?
      }
    };

    let app_interface_port = {
      let app_interfaces = ws.list_app_interfaces().await.map_err(|e| {
        LaunchHolochainError::CouldNotConnectToConductor(format!(
          "Could not list app interfaces: {:?}",
          e
        ))
      })?;

      if app_interfaces.len() > 0 {
        app_interfaces[0]
      } else {
        let free_port = portpicker::pick_unused_port().expect("No ports free");

        ws.attach_app_interface(free_port).await.or(Err(
          LaunchHolochainError::CouldNotConnectToConductor("Could not attach app interface".into()),
        ))?;
        free_port
      }
    };

    Ok(HolochainManager {
      version,
      ws,
      admin_interface_port: config.admin_port,
      app_interface_port,
      command_child,
    })
  }

  pub fn admin_interface_port(&self) -> u16 {
    self.admin_interface_port
  }

  pub fn app_interface_port(&self) -> u16 {
    self.app_interface_port
  }

  pub fn kill(mut self) -> Result<(), String> {
    self.ws.close();
    self
      .command_child
      .kill()
      .map_err(|err| format!("Could not kill the holochain process: {}", err))?;

    Ok(())
  }

  pub async fn install_app(
    &mut self,
    app_id: String,
    app_bundle: AppBundle,
    network_seed: Option<String>,
    membrane_proofs: HashMap<String, MembraneProof>,
    agent_pub_key: Option<AgentPubKey>,
  ) -> Result<(), String> {
    let agent_key = match agent_pub_key {
      Some(pub_key) => Ok(pub_key),
      None => self
        .ws
        .generate_agent_pub_key()
        .await
        .map_err(|err| format!("Error generating public key: {:?}", err)),
    }?;

    // TODO: make this more performant
    // We could be passing the app bundle path directly if what we want to install is a headless app
    // Also consider if the AppBundleSource::Path is a viable alternative in production mode
    let now = SystemTime::now();

    let path = temp_dir().join(format!("app_to_install{:?}.webhapp", now));

    app_bundle
      .write_to_file(&path)
      .await
      .map_err(|err| format!("Could not write app bundle to temp file: {}", err))?;

    let payload = InstallAppPayload {
      source: AppBundleSource::Path(path),
      agent_key,
      installed_app_id: Some(app_id.clone().into()),
      membrane_proofs,
      network_seed,
    };
    self
      .ws
      .install_app(payload)
      .await
      .map_err(|err| format!("Error install hApp bundle: {:?}", err))?;

    self
      .ws
      .enable_app(app_id.into())
      .await
      .map_err(|err| format!("Error enabling app: {:?}", err))?;

    Ok(())
  }

  pub async fn uninstall_app(&mut self, app_id: String) -> Result<(), String> {
    self
      .ws
      .uninstall_app(app_id.into())
      .await
      .map_err(|err| format!("Error uninstalling app: {:?}", err))?;

    Ok(())
  }

  pub async fn enable_app(&mut self, app_id: String) -> Result<(), String> {
    self
      .ws
      .enable_app(app_id.into())
      .await
      .map_err(|err| format!("Error enabling app: {:?}", err))?;

    Ok(())
  }

  // pub async fn start_app(&mut self, app_id: String) -> Result<(), String> {
  //   self
  //     .ws
  //     .start_app(app_id.into())
  //     .await
  //     .map_err(|err| format!("Error starting app: {:?}", err))?;

  //   Ok(())
  // }

  pub async fn disable_app(&mut self, app_id: String) -> Result<(), String> {
    self
      .ws
      .disable_app(app_id.into())
      .await
      .map_err(|err| format!("Error disabling app: {:?}", err))?;

    Ok(())
  }

  pub async fn delete_clone(&mut self, app_id: String, cell_id: CellId) -> Result<(), String> {
    let clone_cell_id = CloneCellId::CellId(cell_id);
    self
      .ws
      .delete_clone_cell(DisableCloneCellPayload {
        app_id,
        clone_cell_id,
      })
      .await
      .map_err(|err| format!("Error deleting cloned cell: {:?}", err))?;

    Ok(())
  }

  pub async fn list_apps(&mut self) -> Result<Vec<AppInfo>, String> {
    let installed_apps = self
      .ws
      .list_apps(None)
      .await
      .map_err(|e| format!("Could not get the currently installed apps: {:?}", e))?;

    Ok(installed_apps)
  }

  pub async fn dump_network_stats(&mut self) -> Result<String, String> {
    let network_stats = self
      .ws
      .dump_network_stats()
      .await
      .map_err(|e| format!("Failed to get network stats: {:?}", e))?;

    Ok(network_stats)
  }
}
