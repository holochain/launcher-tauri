use std::path::{Path, PathBuf};

use std::process::Command;
use std::thread::JoinHandle;
use std::time::Duration;
use std::process::{Stdio};

mod utils;
mod cmds;
pub mod cli;
pub mod error;

pub use cli::HcLaunch;
use crate::cmds::CreateInput;
use holochain_cli_sandbox::cmds::Create;


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

    println!("#*#*# Starting Tauri Windows #*#*#");
    let output = command
      .stdout(Stdio::inherit())
      .output()
      .expect("failed to execute process");

    println!("hc-launch-tauri output: {:?}", String::from_utf8(output.stdout));
  })

}


pub async fn generate_agents_sb(holochain_path: PathBuf, happ: PathBuf, create_input: CreateInput) -> anyhow::Result<()> {

  // clean existing sandboxes
  holochain_cli_sandbox::save::clean(std::env::current_dir()?, Vec::new())?;

  let happ = holochain_cli_sandbox::bundles::parse_happ(Some(happ))?;

  // create a new random id to identify the sandboxes and be
  // able to retrieve the directory to the lair-keystore of each sandbox
  let sandbox_identifier = nanoid::nanoid!();

  let mut directories = Vec::new();

  for agent in 0..create_input.num_sandboxes {
    directories.push(PathBuf::from(format!("{}_Agent-{}", sandbox_identifier, agent)))
  }

  let create = Create {
    num_sandboxes: create_input.num_sandboxes,
    network: create_input.network,
    root: create_input.root,
    directories,
  };

  let app_id = String::from("test-app");

  // holochain_util::pw::pw_set_piped(true);

  // pass dummy lair password
  // let mut echo_child = Command::new("echo")
  //   .arg("pass")
  //   .stdout(Stdio::piped())
  //   .spawn()
  //   .expect("failed to execute echo");

  // println!("pass");

  let paths = holochain_cli_sandbox::sandbox::default_n(
    &holochain_path,
    create,
    happ,
    app_id
  ).await?;


  holochain_cli_sandbox::save::save(std::env::current_dir()?, paths.clone())?;

  let run: Option<Vec<u16>> = Some(vec![]);
  let force_admin_ports: Vec<u16> = vec![];

  if let Some(ports) = run {
    let holochain_path = holochain_path.clone();
    let force_admin_ports = force_admin_ports.clone();
    tokio::task::spawn(async move {
        if let Err(e) =
            run_n(&holochain_path, paths, ports, force_admin_ports).await
        {
            tracing::error!(failed_to_run = ?e);
        }
    });
    // tokio::signal::ctrl_c().await?;
    // holochain_cli_sandbox::save::release_ports(std::env::current_dir()?).await?;
  }

  Ok(())
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


// copied over from hc_sanbox because it's not public (https://github.com/holochain/holochain/blob/03f315be92991f374cba341d210340f7e1141578/crates/hc_sandbox/src/cli.rs#L190)
async fn run_n(
  holochain_path: &Path,
  paths: Vec<PathBuf>,
  app_ports: Vec<u16>,
  force_admin_ports: Vec<u16>,
) -> anyhow::Result<()> {
  let run_holochain = |holochain_path: PathBuf, path: PathBuf, ports, force_admin_port| async move {
      holochain_cli_sandbox::run::run(&holochain_path, path, ports, force_admin_port).await?;
      Result::<_, anyhow::Error>::Ok(())
  };
  let mut force_admin_ports = force_admin_ports.into_iter();
  let mut app_ports = app_ports.into_iter();
  let jhs = paths
      .into_iter()
      .zip(std::iter::repeat_with(|| force_admin_ports.next()))
      .zip(std::iter::repeat_with(|| app_ports.next()))
      .map(|((path, force_admin_port), app_port)| {
          let f = run_holochain(
              holochain_path.to_path_buf(),
              path,
              app_port.map(|p| vec![p]).unwrap_or_default(),
              force_admin_port,
          );
          tokio::task::spawn(f)
      });
  futures::future::try_join_all(jhs).await?;
  Ok(())
}

  // receives .webhapp file

  // splits it in UI and .happ, and creates temporary folders for the two

  // run hc sandbox with the .happ file

  // extract the keystore path from the generated .hc files

  // create lair client to sign zome calls with

  // store admin and app websocket ports somewhere

  // pick up app and admin websocket ports and create tauri window that serves from temporary UI folder




