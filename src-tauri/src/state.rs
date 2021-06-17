use std::{collections::HashMap, sync::{Arc, Mutex}, time::SystemTime};
// use tauri::api::process::CommandChild;

#[derive(Clone)]
pub struct HolochainLauncherState {
//  pub caddy_processes: Arc<Mutex<HashMap<String, CommandChild>>>,

  pub logs: Arc<Mutex<HashMap<usize, String>>>,
}

#[tauri::command]
pub fn get_logs(state: tauri::State<HolochainLauncherState>) -> HashMap<usize, String> {
  let l = state
    .inner()
    .logs
    .to_owned()
    .as_ref()
    .lock()
    .unwrap()
    .clone();

  l
}

impl HolochainLauncherState {
  pub fn new() -> Self {
    HolochainLauncherState {
      // caddy_processes: Arc::new(Mutex::new(HashMap::new())),
      logs: Arc::new(Mutex::new(HashMap::new())),
    }
  }

  pub fn log(&self, log: String) -> () {
    let mut logs = self.logs.lock().unwrap();

    let now = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap();

    logs.insert(now.as_millis() as usize, log);
  }
/* 
  pub fn terminate_all_children(&self) -> () {
    let caddy_proccesses = Arc::clone(&self.caddy_processes);
    let c = caddy_proccesses.lock().unwrap();
    let caddy_proccesses = *c;

    let mut children_processes: Vec<CommandChild> = caddy_proccesses.values().map(|a| *a).collect();

    let hc = Arc::clone(&self.holochain_process);
    let mut hc = hc.lock().unwrap();
    if let Some(process) = (*hc). {
      children_processes.push(process);
    }

    let lair = Arc::clone(&self.holochain_process);
    let mut lair = lair.lock().unwrap();
    if let Some(process) = *lair {
      children_processes.push(process);
    }

    for child_process in children_processes.into_iter() {
      if let Err(error) = child_process.kill() {
        println!("Error killing leftover child: {:?}", error);
      }
    }
  } */
}
