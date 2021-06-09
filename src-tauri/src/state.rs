use std::{process::Child, sync::{Arc, Mutex}};

pub struct HolochainLauncherState {
  pub child_processes: Arc<Mutex<Vec<Child>>>,
}
