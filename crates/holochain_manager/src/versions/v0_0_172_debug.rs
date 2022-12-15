use std::{path::PathBuf, sync::Arc};
use url2::Url2;

// NEW_VERSION: Import the exact same types but from the new crates
use holochain_conductor_api_0_0_172DEBUG::{
  conductor::{ConductorConfig, KeystoreConfig},
  AdminInterfaceConfig, InterfaceDriver,
};
use holochain_p2p_0_0_172DEBUG::kitsune_p2p::{KitsuneP2pConfig, ProxyConfig, TransportConfig, dependencies::kitsune_p2p_types::config::tuning_params_struct::KitsuneP2pTuningParams};

use super::{version_manager::VersionManager, HdkVersion, HdiVersion, common::{proxy_url, boostrap_service}};

pub struct HolochainV0_0_172DEBUG;

impl VersionManager for HolochainV0_0_172DEBUG {
  // NEW_VERSION: Careful! Indicate here which HDK version comes bundled with this Holochain version
  fn hdk_version(&self) -> HdkVersion {
    HdkVersion::V0_0_160
  }

  // NEW_VERSION: Careful! Indicate here which HDI version comes bundled with this Holochain version
  fn hdi_version(&self) -> HdiVersion {
    HdiVersion::V0_1_8
  }

  // NEW_VERSION: Duplicate and change whatever config is necessary to change
  fn initial_config(
    &self,
    admin_port: u16,
    conductor_environment_path: PathBuf,
    keystore_connection_url: Url2,
  ) -> String {
    let mut network_config = KitsuneP2pConfig::default();
    network_config.bootstrap_service = Some(boostrap_service());

    let tuning_params = KitsuneP2pTuningParams::default();

    network_config.tuning_params = Arc::new(tuning_params);

    network_config.transport_pool.push(TransportConfig::Proxy {
      sub_transport: Box::new(TransportConfig::Quic {
        bind_to: None,
        override_host: None,
        override_port: None,
      }),
      proxy_config: ProxyConfig::RemoteProxyClient {
        proxy_url: proxy_url(),
      },
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
      chc_namespace: None,
      db_sync_strategy: Default::default(),
    };

    serde_yaml::to_string(&config).expect("Could not convert conductor config to string")
  }

}
