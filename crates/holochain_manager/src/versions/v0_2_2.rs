use std::{path::PathBuf, sync::Arc};
use url2::Url2;

// NEW_VERSION: Import the exact same types but from the new crates
use holochain_conductor_api_0_2_2::{
  conductor::{ConductorConfig, KeystoreConfig},
  AdminInterfaceConfig, InterfaceDriver,
};
use holochain_p2p_0_2_2::kitsune_p2p::{
  dependencies::kitsune_p2p_types::config::tuning_params_struct::KitsuneP2pTuningParams,
  KitsuneP2pConfig, TransportConfig,
};

use super::{
  common::{bootstrap_service, signaling_server},
  version_manager::VersionManager,
  HdiVersion, HdkVersion,
};

pub struct HolochainV0_2_2;

impl VersionManager for HolochainV0_2_2 {
  // NEW_VERSION: Careful! Indicate here which HDK version comes bundled with this Holochain version
  fn hdk_version(&self) -> HdkVersion {
    HdkVersion::V0_2_2
  }

  // NEW_VERSION: Careful! Indicate here which HDI version comes bundled with this Holochain version
  fn hdi_version(&self) -> HdiVersion {
    HdiVersion::V0_3_2
  }

  // NEW_VERSION: Duplicate and change whatever config is necessary to change
  fn initial_config(
    &self,
    admin_port: u16,
    conductor_environment_path: PathBuf,
    keystore_connection_url: Url2,
    bootstrap_server_url: Option<String>,
    signaling_server_url: Option<String>,
  ) -> String {
    let mut network_config = KitsuneP2pConfig::default();
    network_config.bootstrap_service = Some(match bootstrap_server_url {
      Some(url) => url2::url2!("{}", url),
      None => bootstrap_service(),
    });

    let tuning_params = KitsuneP2pTuningParams::default();

    network_config.tuning_params = Arc::new(tuning_params);

    network_config.transport_pool.push(TransportConfig::WebRTC {
      signal_url: match signaling_server_url {
        Some(url) => url,
        None => signaling_server(),
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
      db_sync_strategy: Default::default(),
      tracing_override: None,
    };

    serde_yaml::to_string(&config).expect("Could not convert conductor config to string")
  }
}
