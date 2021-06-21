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
    println!("Log: {}", log);

    let mut logs = self.logs.lock().unwrap();

    let now = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap();

    logs.insert(now.as_millis() as usize, log);
  }

}
