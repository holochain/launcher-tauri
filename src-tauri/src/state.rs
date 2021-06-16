use std::{
  collections::HashMap,
  process::Child,
  sync::{Arc, Mutex},
  time::SystemTime,
};


#[derive(Clone)]
pub struct HolochainLauncherState {
  pub child_processes: Arc<Mutex<Vec<Child>>>,
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
  pub fn log(&self, log: String) -> () {
    let mut logs = self.logs.lock().unwrap();

    let now = SystemTime::now()
      .duration_since(SystemTime::UNIX_EPOCH)
      .unwrap();

    logs.insert(now.as_millis() as usize, log);
  }

  pub fn add_child_process(&self, child: Child) -> () {
    self.child_processes.lock().unwrap().push(child);
  }

  pub fn terminate_all_children(&self) -> () {
    let mut inner_state = self.child_processes.lock().unwrap();
    let child_processes: &mut Vec<Child> = inner_state.as_mut();
    for child_process in child_processes.into_iter() {
      if let Err(error) = child_process.kill() {
        println!("Error killing leftover child: {:?}", error);
      }
    }
  }
}
