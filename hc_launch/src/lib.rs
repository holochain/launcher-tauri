use std::env;
use std::path::PathBuf;

use std::process::Command;
use std::thread::JoinHandle;
use std::time::Duration;
use std::process::{Stdio};

mod utils;
pub mod cli;
pub mod error;

pub use cli::HcLaunch;


pub fn launch_tauri(ui_path: Option<PathBuf>, watch: bool) -> JoinHandle<()> {

  std::thread::spawn(move || {
    // todo! instead listen for `hc-sandbox: Connected successfully to a running holochain` n times where n is the number of agents
    // and then start the launch process
    println!("Wait for 15 seconds before launching the tauri windows to make sure the conductors are ready.");
    std::thread::sleep(Duration::from_millis(15000));

    let mut command = Command::new("hc-launch-tauri");

    if let Some(path) = ui_path {
      command.args(["--ui-path", path.to_str().unwrap()]);
    }

    if watch {
      command.arg("--watch");
    }

    println!("#*#*# hc-launch-tauri #*#*#");
    let output = command
      .stdout(Stdio::inherit())
      .output()
      .expect("failed to execute process");

    println!("hc-launch-tauri output: {:?}", String::from_utf8(output.stdout));
  })

}


pub fn generate_agents(happ_path: PathBuf, agents: u32, network: Option<String>) -> JoinHandle<()> {

  let _output = Command::new("hc")
    .args(["s", "clean"])
    .output()
    .expect("failed to execute process");

  // create a new random id to identify the sandboxes and be
  // able to retrieve the directory to the lair-keystores
  let sandbox_identifier = nanoid::nanoid!();

  launch_happ(
    &happ_path,
    Some(String::from("test-app")),
    agents,
    Some(sandbox_identifier),
    network,
  )

}


pub fn launch_happ(
  happ_path: &PathBuf,
  app_id: Option<String>,
  n_sandboxes: u32,
  sandbox_identifier: Option<String>, // if provided, the sandbox gets the name [sandbox_identifier]_[app_id]_[agent_number]
  network: Option<String>,
)-> JoinHandle<()> {

  // pass dummy lair password
  let echo_child = Command::new("echo")
    .arg("pass")
    .stdout(Stdio::piped())
    .spawn()
    .expect("failed to execute echo");

  let mut command = Command::new("hc");

  command.args(["s", "--piped", "generate", happ_path.to_str().unwrap(), "--run"]);

  if let Some(a) = app_id {
    command.args(["-a", a.as_str()]);
  }

  // sanbox_identifier's are used to deduce the path to the lair-keystore of each sandbox
  if let Some(id) = sandbox_identifier {
    command.arg("-d");
    for i in 0..n_sandboxes {
      command.arg(format!("{}_Agent-{}",id, i).as_str());
    }
  }

  command.args(["-n", format!("{}", n_sandboxes).as_str(), "network"]);

  if let Some(nw) = network {
    command.arg(nw.as_str());
  }

  std::thread::spawn(move ||  {
    command
      .stdin(Stdio::from(echo_child.stdout.unwrap()))
      .stdout(Stdio::inherit())
      .output()
      .expect("failed to execute process");
  })
}



  // receives .webhapp file

  // splits it in UI and .happ, and creates temporary folders for the two

  // run hc sandbox with the .happ file

  // extract the keystore path from the generated .hc files

  // create lair client to sign zome calls with

  // store admin and app websocket ports somewhere

  // pick up app and admin websocket ports and create tauri window that serves from temporary UI folder




