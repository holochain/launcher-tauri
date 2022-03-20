use std::collections::HashMap;

use async_trait::async_trait;
use holochain_client_0_0_130::{AdminWebsocket, AppStatusFilter, InstallAppBundlePayload};
use holochain_conductor_api_0_0_130::{
  conductor::{ConductorConfig, KeystoreConfig},
  AdminInterfaceConfig, InterfaceDriver,
};
use holochain_p2p_0_0_130::kitsune_p2p::{KitsuneP2pConfig, ProxyConfig, TransportConfig};
use holochain_types_0_0_130::prelude::{AppBundle, AppBundleSource, SerializedBytes};

use crate::{
  holochain_version::HolochainVersion,
  managers::{file_system::FileSystemManager, holochain::conductor::ConductorManager},
};

pub struct ConductorManagerV0_0_130 {
  ws: AdminWebsocket,
}

#[async_trait]
impl ConductorManager for ConductorManagerV0_0_130 {
  type ConductorConfig = ConductorConfig;
  type AdminWebsocket = AdminWebsocket;

  fn holochain_version() -> HolochainVersion {
    HolochainVersion::V0_0_130
  }

  async fn connect(admin_port: u16) -> Result<Self, String> {
    let ws = AdminWebsocket::connect(format!("ws://localhost:{}", admin_port))
      .await
      .or(Err(String::from("Could not connect to conductor")))?;

    Ok(ConductorManagerV0_0_130 { ws })
  }

  fn initial_config(admin_port: u16) -> ConductorConfig {
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

    let fs_manager = FileSystemManager::new(Self::holochain_version());

    ConductorConfig {
      environment_path: fs_manager.conductor_data_path().into(),
      dpki: None,
      keystore: KeystoreConfig::LairServerLegacyDeprecated {
        keystore_path: Some(fs_manager.keystore_data_path()),
        danger_passphrase_insecure_from_config: "test-passphrase".to_string(),
      },
      admin_interfaces: Some(vec![AdminInterfaceConfig {
        driver: InterfaceDriver::Websocket { port: admin_port },
      }]),
      network: Some(network_config),
      db_sync_strategy: Default::default(),
    }
  }

  fn overwrite_config(conductor_config: ConductorConfig, admin_port: u16) -> ConductorConfig {
    let fs_manager = FileSystemManager::new(Self::holochain_version());

    let mut config = conductor_config.clone();

    config.admin_interfaces = Some(vec![AdminInterfaceConfig {
      driver: InterfaceDriver::Websocket { port: admin_port },
    }]);
    config.environment_path = fs_manager.conductor_data_path().into();
    config.keystore = KeystoreConfig::LairServerLegacyDeprecated {
      keystore_path: Some(fs_manager.keystore_data_path()),
      danger_passphrase_insecure_from_config: "test-passphrase".to_string(),
    };

    config
  }

  fn get_current_conductor_config() -> Result<ConductorConfig, String> {
    ConductorConfig::load_yaml(
      &FileSystemManager::new(Self::holochain_version()).conductor_config_path(),
    )
    .map_err(|err| format!("Unable to open the conductor config file: {}", err))
  }

  fn get_admin_port_from_conductor_config() -> Result<Option<u16>, String> {
    let current_config = Self::get_current_conductor_config()?;
    if let Some(admin_interfaces) = current_config.admin_interfaces {
      if let Some(interface) = admin_interfaces.get(0) {
        let InterfaceDriver::Websocket { port } = interface.driver;
        return Ok(Some(port));
      }
    }

    Ok(None)
  }

  /** */
  async fn get_app_port(&mut self) -> Result<u16, String> {
    let app_interfaces = self
      .ws
      .list_app_interfaces()
      .await
      .or(Err(String::from("Could not list app interfaces")))?;

    Ok(app_interfaces[0])
  }

  // If there are no app_interfaces attached, attach one
  async fn setup_conductor(&mut self) -> Result<(), String> {
    let app_interfaces = self
      .ws
      .list_app_interfaces()
      .await
      .or(Err(String::from("Could not list app interfaces")))?;

    if app_interfaces.len() == 0 {
      let free_port = portpicker::pick_unused_port().expect("No ports free");

      self
        .ws
        .attach_app_interface(free_port)
        .await
        .or(Err(String::from("Could not attach app interface")))?;
      log::info!("Attached app interface to {}", free_port);
    }

    Ok(())
  }

  async fn install_app(
    &mut self,
    app_id: &String,
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

  async fn uninstall_app(&mut self, app_id: &String) -> Result<(), String> {
    self
      .ws
      .uninstall_app(app_id.into())
      .await
      .map_err(|err| format!("Error uninstalling app: {:?}", err))?;

    Ok(())
  }

  async fn enable_app(&mut self, app_id: &String) -> Result<(), String> {
    self
      .ws
      .enable_app(app_id.into())
      .await
      .map_err(|err| format!("Error enabling app: {:?}", err))?;

    Ok(())
  }

  async fn disable_app(&mut self, app_id: &String) -> Result<(), String> {
    self
      .ws
      .disable_app(app_id.into())
      .await
      .map_err(|err| format!("Error disabling app: {:?}", err))?;

    Ok(())
  }

  async fn list_running_apps(&mut self) -> Result<Vec<String>, String> {
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
