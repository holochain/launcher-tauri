use lair_keystore_manager::{versions::v0_0_9::LairKeystoreManagerV0_0_9, LairKeystoreManager};
use portpicker;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use async_trait::async_trait;
use holochain_client_0_0_130::{AdminWebsocket, AppStatusFilter, InstallAppBundlePayload};
use holochain_conductor_api_0_0_130::{
  conductor::{ConductorConfig, KeystoreConfig},
  AdminInterfaceConfig, InterfaceDriver,
};
use holochain_p2p_0_0_130::kitsune_p2p::{KitsuneP2pConfig, ProxyConfig, TransportConfig};
use holochain_types_0_0_130::prelude::{AppBundle, AppBundleSource, SerializedBytes};

use super::{launch::launch_holochain_process, utils::create_dir_if_necessary, HolochainVersion};

use crate::{
  app_manager::AppManager,
  config::LaunchHolochainConfig, error::LaunchHolochainError, holochain_manager::HolochainManager,
};

pub struct HolochainManagerV0_0_130 {
  ws: AdminWebsocket,
}

#[async_trait]
impl HolochainManager for HolochainManagerV0_0_130 {
  fn holochain_version() -> HolochainVersion {
    HolochainVersion::V0_0_130
  }

  async fn launch(config: LaunchHolochainConfig) -> Result<Self, LaunchHolochainError> {
    create_dir_if_necessary(&config.conductor_config_path);
    create_dir_if_necessary(&config.environment_path);
    create_dir_if_necessary(&config.keystore_path);

    let new_conductor_config: ConductorConfig = conductor_config(
      config.admin_port,
      config.conductor_config_path.clone(),
      config.environment_path,
      config.keystore_path.clone(),
    );

    let serde_config = serde_yaml::to_string(&new_conductor_config)
      .expect("Could not serialize initial conductor config");

    fs::write(config.conductor_config_path.clone(), serde_config)
      .expect("Could not write conductor config");

    LairKeystoreManagerV0_0_9::launch(config.log_level, config.keystore_path)
      .await
      .map_err(|err| LaunchHolochainError::LaunchKeystoreError(err))?;

    launch_holochain_process(
      config.log_level,
      Self::holochain_version(),
      config.conductor_config_path,
    )
    .map_err(|err| LaunchHolochainError::LaunchHolochainError(err))?;

    let ws = AdminWebsocket::connect(format!("ws://localhost:{}", config.admin_port))
      .await
      .map_err(|err| LaunchHolochainError::CouldNotConnectToConductor(format!("{}", err)))?;

    Ok(HolochainManagerV0_0_130 { ws })
  }

  async fn get_app_interface_port(&mut self) -> Result<u16, String> {
    let app_interfaces = self
      .ws
      .list_app_interfaces()
      .await
      .or(Err(String::from("Could not list app interfaces")))?;

    if app_interfaces.len() > 0 {
      return Ok(app_interfaces[0]);
    }

    let free_port = portpicker::pick_unused_port().expect("No ports free");

    self
      .ws
      .attach_app_interface(free_port)
      .await
      .or(Err(String::from("Could not attach app interface")))?;

    Ok(free_port)
  }
}

#[async_trait]
impl AppManager for HolochainManagerV0_0_130 {
  type RunningApps = Vec<String>;

  async fn install_app(
    &mut self,
    app_id: String,
    app_bundle: AppBundle,
    uid: Option<String>,
    membrane_proofs: HashMap<String, SerializedBytes>,
  ) -> Result<(), String> {
    let new_key = self
      .ws
      .generate_agent_pub_key()
      .await
      .map_err(|err| format!("Error generating public key: {:?}", err))?;

    let payload = InstallAppBundlePayload {
      source: AppBundleSource::Bundle(app_bundle),
      agent_key: new_key,
      installed_app_id: Some(app_id.clone().into()),
      membrane_proofs,
      uid,
    };
    self
      .ws
      .install_app_bundle(payload)
      .await
      .map_err(|err| format!("Error install hApp bundle: {:?}", err))?;

    self
      .ws
      .enable_app(app_id.into())
      .await
      .map_err(|err| format!("Error enabling app: {:?}", err))?;

    Ok(())
  }

  async fn uninstall_app(&mut self, app_id: String) -> Result<(), String> {
    self
      .ws
      .uninstall_app(app_id.into())
      .await
      .map_err(|err| format!("Error uninstalling app: {:?}", err))?;

    Ok(())
  }

  async fn enable_app(&mut self, app_id: String) -> Result<(), String> {
    self
      .ws
      .enable_app(app_id.into())
      .await
      .map_err(|err| format!("Error enabling app: {:?}", err))?;

    Ok(())
  }

  async fn disable_app(&mut self, app_id: String) -> Result<(), String> {
    self
      .ws
      .disable_app(app_id.into())
      .await
      .map_err(|err| format!("Error disabling app: {:?}", err))?;

    Ok(())
  }

  async fn get_running_apps(&mut self) -> Result<Vec<String>, String> {
    let active_apps = self
      .ws
      .list_apps(Some(AppStatusFilter::Running))
      .await
      .or(Err("Could not get the currently active apps"))?;

    let active_app_ids = active_apps
      .into_iter()
      .map(|a| a.installed_app_id)
      .collect();

    Ok(active_app_ids)
  }

}

fn conductor_config(
  admin_port: u16,
  conductor_config_path: PathBuf,
  environment_path: PathBuf,
  keystore_path: PathBuf,
) -> ConductorConfig {
  if let Ok(current_config_str) = fs::read_to_string(conductor_config_path) {
    if let Ok(conductor_config) =
      serde_yaml::from_str::<ConductorConfig>(String::from(current_config_str).as_str())
    {
      return overwrite_admin_port(conductor_config, admin_port);
    }
  }
  initial_config(admin_port, environment_path, keystore_path)
}

fn initial_config(
  admin_port: u16,
  conductor_environment_path: PathBuf,
  keystore_path: PathBuf,
) -> ConductorConfig {
  let mut network_config = KitsuneP2pConfig::default();
  network_config.bootstrap_service = Some(url2::url2!("https://bootstrap.holo.host"));
  network_config.transport_pool.push(TransportConfig::Proxy {
            sub_transport: Box::new(TransportConfig::Quic {
                bind_to: None,
                override_host: None,
                override_port: None,
            }),
            proxy_config: ProxyConfig::RemoteProxyClientFromBootstrap {
                bootstrap_url: url2::url2!("https://bootstrap.holo.host"),
                fallback_proxy_url: Some(url2::url2!("kitsune-proxy://SYVd4CF3BdJ4DS7KwLLgeU3_DbHoZ34Y-qroZ79DOs8/kitsune-quic/h/165.22.32.11/p/5779/--")),
            },
        });

  ConductorConfig {
    environment_path: conductor_environment_path.into(),
    dpki: None,
    keystore: KeystoreConfig::LairServerLegacyDeprecated {
      keystore_path: Some(keystore_path),
      danger_passphrase_insecure_from_config: "test-passphrase".to_string(),
    },
    admin_interfaces: Some(vec![AdminInterfaceConfig {
      driver: InterfaceDriver::Websocket { port: admin_port },
    }]),
    network: Some(network_config),
    db_sync_strategy: Default::default(),
  }
}

fn overwrite_admin_port(conductor_config: ConductorConfig, admin_port: u16) -> ConductorConfig {
  let mut config = conductor_config.clone();

  config.admin_interfaces = Some(vec![AdminInterfaceConfig {
    driver: InterfaceDriver::Websocket { port: admin_port },
  }]);

  config
}
