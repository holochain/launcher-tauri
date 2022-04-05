use lair_keystore_manager::{utils::create_dir_if_necessary, versions::LairKeystoreVersion};
use portpicker;
use std::fs;
use std::path::PathBuf;
use std::{collections::HashMap, time::Duration};
use tauri::api::process::CommandChild;
use url2::Url2;

use async_trait::async_trait;
use holochain_client_0_0_132::{AdminWebsocket, InstallAppBundlePayload, InstalledAppInfo};
use holochain_conductor_api_0_0_132::{
  conductor::{ConductorConfig, KeystoreConfig},
  AdminInterfaceConfig, InterfaceDriver,
};
use holochain_p2p_0_0_132::kitsune_p2p::{KitsuneP2pConfig, ProxyConfig, TransportConfig};
use holochain_types_0_0_132::prelude::{AppBundle, AppBundleSource, SerializedBytes};

use super::{launch::launch_holochain_process, HolochainVersion};

use crate::{
  config::LaunchHolochainConfig, error::LaunchHolochainError, holochain_manager::HolochainManager,
};

pub struct HolochainManagerV0_0_132 {
  ws: AdminWebsocket,
  admin_interface_port: u16,
  app_interface_port: u16,
  command_child: CommandChild,
}

impl HolochainManagerV0_0_132 {
  pub async fn launch(
    config: LaunchHolochainConfig,
    password: String,
  ) -> Result<Self, LaunchHolochainError> {
    let conductor_config_path = config.config_environment_path.join("conductor-config.yaml");
    create_dir_if_necessary(&config.config_environment_path)?;
    create_dir_if_necessary(&config.environment_path)?;

    let new_conductor_config: ConductorConfig = conductor_config(
      config.admin_port,
      conductor_config_path.clone(),
      config.environment_path,
      config.keystore_connection_url.clone(),
    );

    let serde_config = serde_yaml::to_string(&new_conductor_config)
      .expect("Could not serialize initial conductor config");

    fs::write(conductor_config_path.clone(), serde_config)
      .expect("Could not write conductor config");

    let command_child = launch_holochain_process(
      config.log_level,
      HolochainVersion::V0_0_132,
      conductor_config_path,
      password,
    )?;

    std::thread::sleep(Duration::from_millis(3000));

    let mut ws = AdminWebsocket::connect(format!("ws://localhost:{}", config.admin_port))
      .await
      .map_err(|err| LaunchHolochainError::CouldNotConnectToConductor(format!("{}", err)))?;

    let app_interface_port = {
      let app_interfaces = ws.list_app_interfaces().await.or(Err(
        LaunchHolochainError::CouldNotConnectToConductor("Could not list app interfaces".into()),
      ))?;

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

    Ok(HolochainManagerV0_0_132 {
      ws,
      admin_interface_port: config.admin_port,
      app_interface_port,
      command_child,
    })
  }
}

#[async_trait]
impl HolochainManager for HolochainManagerV0_0_132 {
  fn holochain_version(&self) -> HolochainVersion {
    HolochainVersion::V0_0_132
  }

  fn lair_keystore_version(&self) -> LairKeystoreVersion {
    LairKeystoreVersion::V0_1_0
  }

  fn admin_interface_port(&self) -> u16 {
    self.admin_interface_port
  }

  fn app_interface_port(&self) -> u16 {
    self.app_interface_port
  }

  fn kill(mut self) -> Result<(), String> {
    self.ws.close();
    self
      .command_child
      .kill()
      .map_err(|err| format!("Could not kill the holochain process: {}", err))?;

    Ok(())
  }

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

  async fn list_apps(&mut self) -> Result<Vec<InstalledAppInfo>, String> {
    let installed_apps = self
      .ws
      .list_apps(None)
      .await
      .or(Err("Could not get the currently installed apps"))?;

    Ok(installed_apps)
  }
}

fn conductor_config(
  admin_port: u16,
  conductor_config_path: PathBuf,
  environment_path: PathBuf,
  keystore_connection_url: Url2,
) -> ConductorConfig {
  if let Ok(current_config_str) = fs::read_to_string(conductor_config_path) {
    if let Ok(conductor_config) =
      serde_yaml::from_str::<ConductorConfig>(String::from(current_config_str).as_str())
    {
      return overwrite_config(conductor_config, admin_port, keystore_connection_url);
    }
  }
  initial_config(admin_port, environment_path, keystore_connection_url)
}

fn initial_config(
  admin_port: u16,
  conductor_environment_path: PathBuf,
  keystore_connection_url: Url2,
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
    keystore: KeystoreConfig::LairServer {
      connection_url: keystore_connection_url,
    },
    admin_interfaces: Some(vec![AdminInterfaceConfig {
      driver: InterfaceDriver::Websocket { port: admin_port },
    }]),
    network: Some(network_config),
    db_sync_strategy: Default::default(),
  }
}

fn overwrite_config(
  conductor_config: ConductorConfig,
  admin_port: u16,
  keystore_connection_url: Url2,
) -> ConductorConfig {
  let mut config = conductor_config.clone();

  config.admin_interfaces = Some(vec![AdminInterfaceConfig {
    driver: InterfaceDriver::Websocket { port: admin_port },
  }]);

  config.keystore = KeystoreConfig::LairServer {
    connection_url: keystore_connection_url,
  };

  config
}
