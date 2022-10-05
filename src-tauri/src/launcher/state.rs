use futures::lock::Mutex;
use holochain_manager::versions::{HolochainVersion, HdiVersion, HdkVersion};
use holochain_web_app_manager::installed_web_app_info::InstalledWebAppInfo;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

use super::{
  config::LauncherConfig,
  error::LauncherError,
  manager::{KeystoreStatus, LauncherManager},
};
use crate::running_state::RunningState;

pub type LauncherState = Arc<Mutex<RunningState<LauncherManager, LauncherError>>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct HolochainInfo {
  pub installed_apps: Vec<InstalledWebAppInfo>,
  pub app_interface_port: u16,
  pub admin_interface_port: u16,
  pub hdi_version: HdiVersion,
  pub hdk_version: HdkVersion,
}

pub type HolochainState = RunningState<HolochainInfo, String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct LauncherStateInfo {
  pub state: RunningState<RunningState<RunningHolochainsStateInfo, KeystoreStatus>, LauncherError>,
  pub config: LauncherConfig,
  pub default_version: HolochainVersion,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RunningHolochainsStateInfo {
  pub versions: HashMap<HolochainVersion, HolochainState>,
  pub custom_binary: Option<HolochainState>,
}
