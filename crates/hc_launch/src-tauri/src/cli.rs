//! Definitions of StructOpt options for use in the CLI

// use holochain_types::prelude::InstalledAppId;
// use std::path::Path;
use std::path::{ Path, PathBuf };
use holochain_cli_sandbox::CmdRunner;
use holochain_cli_sandbox::calls::{AddAppWs, attach_app_interface};
use holochain_cli_sandbox::run::run_async;
use structopt::StructOpt;
use holochain_types::prelude::InstalledAppId;
use tokio::task::JoinHandle;

use crate::launch_tauri::launch_tauri;
use crate::utils;
use crate::error::HcLaunchError;
use holochain_cli_sandbox::cmds::Create;

// const DEFAULT_APP_ID: &str = "test-app";

#[derive(Debug, StructOpt)]
/// Helper for launching holochain apps in a holochain-launcher environment for testing and development purposes.
///
pub struct HcLaunch {

    /// Instead of the normal "interactive" passphrase mode,
    /// collect the passphrase by reading stdin to the end.
    #[structopt(long)]
    piped: bool,

    /// Set the path to the holochain binary.
    #[structopt(long, env = "HC_HOLOCHAIN_PATH", default_value = "holochain")]
    holochain_path: PathBuf,

    /// Path to .webhapp of .happ file to launch.
    path: Option<PathBuf>,

    // #[structopt(long)]
    // /// Port of the UI
    // ui_port: Option<u16>,

    #[structopt(long)]
    /// path to the UI. Required if a .happ file is passed.
    ui_path: Option<PathBuf>,

    /// Watch for file changes in the UI folder. Requires --ui-path to be specified.
    #[structopt(long, short)]
    watch: bool,

    /// (flattened)
    #[structopt(flatten)]
    create: Create,

}


impl HcLaunch {
  /// Run this command
  pub async fn run(self) -> anyhow::Result<()> {
    holochain_util::pw::pw_set_piped(self.piped);

    match self.path {
      Some(p) => {
        match p.extension() {
          Some(extension) => {
            match extension.to_str().unwrap() {
              "webhapp" => {
                // generate temp folder
                let temp_dir = tempdir::TempDir::new("hc_launch").unwrap();
                let temp_folder = temp_dir.path().to_path_buf();

                // unzip the webhapp, prepare UI etc.
                match utils::prepare_webapp::read_and_prepare_webapp(&p, &temp_folder).await {
                  Ok(()) => (),
                  Err(e) => {
                    println!("Failed to read and prepare webhapp: {:?}", e);
                    panic!("Failed to read and prepare webhapp");
                  }
                };

                // generate agents
                let happ_path = temp_folder.join("happ.happ");

                // clean existing sandboxes
                holochain_cli_sandbox::save::clean(std::env::current_dir()?, Vec::new())?;

                // spawn sandboxes
                let _join_handles = spawn_sandboxes(
                  &self.holochain_path,
                  happ_path,
                  self.create,
                  String::from("test-app"),
                ).await?;

                // spawn tauri windows
                let ui_path = match self.ui_path {
                  Some(p) => p,
                  None => temp_folder.join("ui").into(), // TODO! switch to tmp directory for ui and .happ
                };

                tauri::async_runtime::spawn(async move {
                  // This stuff is never being called :/
                  tokio::signal::ctrl_c().await.unwrap();
                  holochain_cli_sandbox::save::release_ports(std::env::current_dir().unwrap()).await.unwrap();
                  temp_dir.close().unwrap();
                  std::process::exit(0);
                });

                launch_tauri(ui_path, self.watch);
              }
              "happ" => {
                match self.ui_path {
                  Some(ui_p) => {

                    // check whether path exists
                    if !ui_p.exists() {
                      return Err(anyhow::Error::from(HcLaunchError::UiPathDoesNotExist(format!("{}", ui_p.to_str().unwrap()))));
                    }

                    // clean existing sandboxes
                    holochain_cli_sandbox::save::clean(std::env::current_dir()?, Vec::new())?;

                    // spawn sandboxes
                    let join_handles = spawn_sandboxes(
                      &self.holochain_path,
                      p,
                      self.create,
                      String::from("test-app"),
                    ).await?;

                    tauri::async_runtime::spawn(async move {
                      // This stuff is never being called :/
                      tokio::signal::ctrl_c().await.unwrap();
                      holochain_cli_sandbox::save::release_ports(std::env::current_dir().unwrap()).await.unwrap();
                      std::process::exit(0);
                    });

                    launch_tauri(ui_p, self.watch);
                  },
                  None => eprintln!("Error: If you provide a path to a .happ file you also need to specify a path to the UI assets via the --ui-path option.\nRun `hc-launch --help` for help."),
                }
              },
              _ => eprintln!("Error: You need to provide a path that points to either a .webhapp or a .happ file."),
            }
          },
          None => eprintln!("Error: You need to provide a path that points to either a .webhapp or a .happ file.")
        }
      },
      None => println!("You need to provide a path that points to either a .webhapp or a .happ file. Auto-detection is not implemented yet.")
    }

    Ok(())
  }

}





async fn spawn_sandboxes(
  holochain_path: &PathBuf,
  happ_path: PathBuf,
  create: Create,
  app_id: InstalledAppId
) -> anyhow::Result<()> {


  let sandbox_paths = generate(
    holochain_path,
    Some(happ_path),
    create,
    app_id,
  ).await?;


  let run: Option<Vec<u16>> = Some(vec![]);
  let force_admin_ports: Vec<u16> = vec![];

  if let Some(ports) = run {
    let holochain_path_clone = holochain_path.clone();
    let force_admin_ports = force_admin_ports.clone();
    tokio::task::spawn(async move {
        if let Err(e) =
            run_n(&holochain_path_clone, sandbox_paths, ports, force_admin_ports).await
        {
            tracing::error!(failed_to_run = ?e);
        }
    });
  }

  Ok(())
}




// copied from hc sandbox because it's private (https://github.com/holochain/holochain/blob/540c2497f778cc004c1e7114662733fe197790cc/crates/hc_sandbox/src/cli.rs#L219)
async fn generate(
  holochain_path: &Path,
  happ: Option<PathBuf>,
  create: Create,
  app_id: InstalledAppId,
) -> anyhow::Result<Vec<PathBuf>> {
  let happ = holochain_cli_sandbox::bundles::parse_happ(happ)?;
  let paths = holochain_cli_sandbox::sandbox::default_n(holochain_path, create, happ, app_id).await?;
  holochain_cli_sandbox::save::save(std::env::current_dir()?, paths.clone())?;
  Ok(paths)
}


// copied over from hc_sanbox because it's not public (https://github.com/holochain/holochain/blob/03f315be92991f374cba341d210340f7e1141578/crates/hc_sandbox/src/cli.rs#L190)
async fn run_n(
  holochain_path: &Path,
  paths: Vec<PathBuf>,
  app_ports: Vec<u16>,
  force_admin_ports: Vec<u16>,
) -> anyhow::Result<()> {
  let run_holochain = |holochain_path: PathBuf, path: PathBuf, ports, force_admin_port| async move {
      run(&holochain_path, path, ports, force_admin_port).await?;
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


// Copied over from hc_sandbox (https://github.com/holochain/holochain/blob/540c2497f778cc004c1e7114662733fe197790cc/crates/hc_sandbox/src/run.rs#L32)
// to make it possible to listen to when conductors are ready
pub async fn run(
  holochain_path: &Path,
  sandbox_path: PathBuf,
  app_ports: Vec<u16>,
  force_admin_port: Option<u16>,
) -> anyhow::Result<()> {
  let (port, mut holochain, mut lair) =
      run_async(holochain_path, sandbox_path.clone(), force_admin_port).await?;
  println!("Running conductor on admin port {}", port);
  for app_port in app_ports {
      let mut cmd = CmdRunner::try_new(port).await?;
      let port = attach_app_interface(
          &mut cmd,
          AddAppWs {
              port: Some(app_port),
          },
      )
      .await?;
      println!("App port attached at {}", port);
  }
  holochain_cli_sandbox::save::lock_live(std::env::current_dir()?, &sandbox_path, port).await?;
  println!("Connected successfully to a running holochain");
  let e = format!("Failed to run holochain at {}", sandbox_path.display());

  holochain.wait().await.expect(&e);
  let _ = lair.kill().await;
  lair.wait().await.expect("Failed to wait on lair-keystore");

  Ok(())
}