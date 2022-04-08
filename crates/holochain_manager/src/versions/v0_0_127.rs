use std::path::PathBuf;
use url2::Url2;

// NEW_VERSION: Import the exact same types but from the new crates
use holochain_conductor_api_0_0_127::{
  conductor::{ConductorConfig, KeystoreConfig},
  AdminInterfaceConfig, InterfaceDriver,
};
use holochain_p2p_0_0_127::kitsune_p2p::{KitsuneP2pConfig, ProxyConfig, TransportConfig};

use super::{version_manager::VersionManager, HdkVersion};

pub struct HolochainV0_0_127;

impl VersionManager for HolochainV0_0_127 {
  // NEW_VERSION: Careful! Indicate here which HDK version comes bundled with this Holochain version
  fn hdk_version(&self) -> HdkVersion {
    HdkVersion::V0_0_123
  }

  // NEW_VERSION: Duplicate and change whatever config is necessary to change
  fn initial_config(
    &self,
    admin_port: u16,
    conductor_environment_path: PathBuf,
    keystore_connection_url: Url2,
  ) -> String {
    let mut network_config = KitsuneP2pConfig::default();
    network_config.bootstrap_service = Some(url2::url2!("https://bootstrap.holo.host"));
    network_config.transport_pool.push(TransportConfig::Proxy {
            sub_transport: Box::new(TransportConfig::Quic {
                bind_to: None,
                override_host: None,
                override_port: None,
            }),
            proxy_config: ProxyConfig::RemoteProxyClient { 
              proxy_url: url2::url2!("kitsune-proxy://SYVd4CF3BdJ4DS7KwLLgeU3_DbHoZ34Y-qroZ79DOs8/kitsune-quic/h/165.22.32.11/p/5779/--") 
            }
        });

    let config = ConductorConfig {
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
    };

    serde_yaml::to_string(&config).expect("Could not convert conductor config to string")
  }

  // NEW_VERSION: You shouldn't have to change this function if the shape 
  // of the configuration concerning the admin port and the keystore connection haven't changed
  fn overwrite_config(
    &self,
    conductor_config_str: String,
    admin_port: u16,
    keystore_connection_url: Url2,
  ) -> String {
    let mut config = serde_yaml::from_str::<ConductorConfig>(conductor_config_str.as_str())
      .expect("Couldn't convert string to conductor config");

    config.admin_interfaces = Some(vec![AdminInterfaceConfig {
      driver: InterfaceDriver::Websocket { port: admin_port },
    }]);

    config.keystore = KeystoreConfig::LairServer {
      connection_url: keystore_connection_url,
    };

    serde_yaml::to_string(&config).expect("Could not convert conductor config to string")
  }
}
