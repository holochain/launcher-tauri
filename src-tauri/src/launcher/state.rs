use futures::lock::Mutex;
use holochain_manager::versions::HolochainVersion;
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

pub type HolochainStateInfo = RunningState<Vec<InstalledWebAppInfo>, String>;

#[derive(Serialize, Deserialize, Debug)]
pub struct LauncherStateInfo {
  pub state: RunningState<
    RunningState<HashMap<HolochainVersion, HolochainStateInfo>, KeystoreStatus>,
    LauncherError,
  >,
  pub config: LauncherConfig,
}
