//! Definitions of StructOpt options for use in the CLI

// use holochain_types::prelude::InstalledAppId;
// use std::path::Path;
use holochain_cli_sandbox::calls::{attach_app_interface, AddAppWs};
use holochain_cli_sandbox::run::run_async;
use holochain_cli_sandbox::CmdRunner;
use holochain_launcher_utils::window_builder::UISource;
use holochain_types::prelude::InstalledAppId;
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use tokio::process::Child;

use crate::launch_tauri::launch_tauri;
use crate::prepare_webapp;
use holochain_cli_sandbox::cmds::Create;


#[derive(Debug, StructOpt)]
/// Helper for launching holochain apps in a Holochain Launcher environment for testing and development purposes.
///
pub struct HcLaunch {
  /// Instead of the normal "interactive" passphrase mode,
  /// collect the passphrase by reading stdin to the end.
  #[structopt(long)]
  piped: bool,

  /// Set the path to the holochain binary.
  #[structopt(long, env = "HC_HOLOCHAIN_PATH", default_value = "holochain")]
  holochain_path: PathBuf,

  /// Path to .webhapp or .happ file to launch. If a .happ file is passed, either
  /// a UI path must be specified via --ui-path or a port pointing to a localhost
  /// server via --ui-port.
  path: Option<PathBuf>,

  #[structopt(long)]
  /// Port pointing to a localhost server that serves your assets.
  /// NOTE: This is only meant for development purposes! Apps can behave differently when
  /// served from a localhost server than when actually running in the Holochain Launcher.
  /// Use the --ui-path flag pointing to your built and bundled files instead or directly pass
  /// the packaged .webhapp to test the actual behavior of your hApp in the Holochain Launcher.
  ui_port: Option<u16>,

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

    let maybe_ui_source = match (self.ui_path.clone(), self.ui_port) {
      (Some(ui_path), None) => Some(UISource::Path(ui_path)),
      (None, Some(ui_port)) => Some(UISource::Port(ui_port)),
      (Some(_ui_path), Some(_ui_port)) => {
        eprintln!("[hc launch] ERROR: You cannot provide both --ui-path and --ui-port.");
        panic!("ERROR: Provided both --ui-path and --ui-port");
      },
      (None, None) => None,
    };

    if let Some(_port) = self.ui_port {
      println!("\n[hc launch] ------ WARNING ------");
      println!(r#"[hc launch] You are running hc launch pointing to a localhost server. This is meant for development purposes
[hc launch] only as apps can behave differently than when actually running in the Holochain Launcher.
[hc launch] To test the real behavior, use --ui-path instead and point to a folder with your built and bundled files
[hc launch] or pass an already packaged .webhapp as an argument."#);
      println!("[hc launch] ---------------------\n");

    }

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
                match prepare_webapp::read_and_prepare_webapp(&p, &temp_folder).await {
                  Ok(()) => (),
                  Err(e) => {
                    println!("[hc launch] Failed to read and prepare webhapp: {:?}", e);
                    panic!("Failed to read and prepare webhapp");
                  }
                };

                // extraxt filename of .webhapp
                let app_id = p.as_path().file_stem().unwrap().to_str().unwrap();
                let happ_file_name = format!("{}.happ", app_id);

                // generate agents
                let happ_path = temp_folder.join(happ_file_name);

                // clean existing sandboxes
                holochain_cli_sandbox::save::clean(std::env::current_dir()?, Vec::new())?;

                // spawn sandboxes
                println!("[hc launch] Spawning sandbox conductors.");
                let _child_processes = spawn_sandboxes(
                  &self.holochain_path,
                  happ_path,
                  self.create,
                  app_id.to_string(),
                ).await?;

                let passphrase = holochain_util::pw::pw_get()?;

                tauri::async_runtime::spawn(async move {
                  tokio::signal::ctrl_c().await.unwrap();
                  holochain_cli_sandbox::save::release_ports(std::env::current_dir().unwrap()).await.unwrap();
                  temp_dir.close().unwrap();
                  std::process::exit(0);
                });


                // spawn tauri windows
                let ui_source = match maybe_ui_source {
                  Some(UISource::Path(path)) => UISource::Path(path),
                  Some(UISource::Port(port)) => UISource::Port(port),
                  None => UISource::Path(temp_folder.join("ui").into()),
                };

                // In case a dedicated ui path is passed, check whether it exists, otherwise wait
                if let Some(ui_p) = self.ui_path {
                  while !ui_p.exists() {
                    println!("[hc launch] You specified a dedicated UI path to use instead of the UI of the .webhapp file but this path does not exist (yet). Waiting before launching tauri windows...");
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    // return Err(anyhow::Error::from(HcLaunchError::UiPathDoesNotExist(format!("{}", ui_p.to_str().unwrap()))));
                  }
                }

                let local_storage_path = temp_folder.join("tauri");

                println!("[hc launch] Launching tauri windows.");
                launch_tauri(ui_source, app_id.to_string(), local_storage_path, self.watch, passphrase);
              }
              "happ" => {
                match maybe_ui_source {
                  Some(ui_source) => {

                    // extraxt filename of .happ
                    let app_id = p.clone().as_path().file_stem().unwrap().to_str().unwrap().to_string();

                    // clean existing sandboxes
                    holochain_cli_sandbox::save::clean(std::env::current_dir()?, Vec::new())?;

                    // spawn sandboxes
                    println!("[hc launch] Spawning sandbox conductors.");
                    let _child_processes = spawn_sandboxes(
                      &self.holochain_path,
                      p,
                      self.create,
                      app_id.clone(),
                    ).await?;

                    tauri::async_runtime::spawn(async move {
                      // This stuff is never being called :/
                      tokio::signal::ctrl_c().await.unwrap();
                      holochain_cli_sandbox::save::release_ports(std::env::current_dir().unwrap()).await.unwrap();
                      std::process::exit(0);
                    });

                    let passphrase = holochain_util::pw::pw_get()?;


                    // In case a ui path is passed, check whether it exists, otherwise wait
                    if let Some(ui_p) = self.ui_path {
                      while !ui_p.exists() {
                        println!("[hc launch] Specified UI path does not exist (yet). Waiting before launching tauri windows...");
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                        // return Err(anyhow::Error::from(HcLaunchError::UiPathDoesNotExist(format!("{}", ui_p.to_str().unwrap()))));
                      }
                    }

                    println!("[hc launch] Launching tauri windows.");

                    // generate temp folder for localStorage
                    let temp_dir = tempdir::TempDir::new("hc_launch").unwrap();
                    let temp_folder = temp_dir.path().to_path_buf();

                    let local_storage_path = temp_folder.join("tauri");

                    launch_tauri(ui_source, app_id, local_storage_path, self.watch, passphrase);
                  },
                  None => eprintln!("[hc launch] Error: If you provide a path to a .happ file you also need to specify eithar a path to the UI assets via the --ui-path option or a port to a server running on localhost using --ui-port.\nRun `hc launch --help` for help."),
                }
              },
              _ => eprintln!("[hc launch] Error: You need to provide a path that points to either a .webhapp or a .happ file.\nRun `hc launch --help` for help."),
            }
          },
          None => eprintln!("[hc launch] Error: You need to provide a path that points to either a .webhapp or a .happ file.\nRun `hc launch --help` for help.")
        }
      },
      None => eprintln!("[hc launch] Error: You need to provide a path that points to either a .webhapp or a .happ file. Auto-detection is not implemented yet.\nRun `hc launch --help` for help.")
    }

    Ok(())
  }
}

async fn spawn_sandboxes(
  holochain_path: &PathBuf,
  happ_path: PathBuf,
  create: Create,
  app_id: InstalledAppId,
) -> anyhow::Result<Vec<(Child, Child)>> {
  let sandbox_paths = generate(holochain_path, Some(happ_path), create, app_id).await?;

  let port = portpicker::pick_unused_port().expect("Cannot find any unused port");
  let force_admin_ports: Vec<u16> = vec![];

  let holochain_path_clone = holochain_path.clone();
  let force_admin_ports = force_admin_ports.clone();
  let result = run_n(
    &holochain_path_clone,
    sandbox_paths,
    vec![port],
    force_admin_ports,
  )
  .await;

  if let Err(e) = &result {
    tracing::error!(failed_to_run = ?e);
  }
  result
}

// copied from hc sandbox because it's private (https://github.com/holochain/holochain/blob/540c2497f778cc004c1e7114662733fe197790cc/crates/hc_sandbox/src/cli.rs#L219)
async fn generate(
  holochain_path: &Path,
  happ: Option<PathBuf>,
  create: Create,
  app_id: InstalledAppId,
) -> anyhow::Result<Vec<PathBuf>> {
  let happ = holochain_cli_sandbox::bundles::parse_happ(happ)?;
  let paths =
    holochain_cli_sandbox::sandbox::default_n(holochain_path, create, happ, app_id).await?;
  holochain_cli_sandbox::save::save(std::env::current_dir()?, paths.clone())?;
  Ok(paths)
}

// copied over from hc_sanbox because it's not public (https://github.com/holochain/holochain/blob/03f315be92991f374cba341d210340f7e1141578/crates/hc_sandbox/src/cli.rs#L190)
async fn run_n(
  holochain_path: &Path,
  paths: Vec<PathBuf>,
  app_ports: Vec<u16>,
  force_admin_ports: Vec<u16>,
) -> anyhow::Result<Vec<(Child, Child)>> {
  let run_holochain = |holochain_path: PathBuf, path: PathBuf, ports, force_admin_port| async move {
    run(&holochain_path, path, ports, force_admin_port).await
  };
  let mut force_admin_ports = force_admin_ports.into_iter();
  let mut app_ports = app_ports.into_iter();

  let jhs = paths
    .into_iter()
    .zip(std::iter::repeat_with(|| force_admin_ports.next()))
    .zip(std::iter::repeat_with(|| app_ports.next()))
    .map(|((path, force_admin_port), app_port)| {
      run_holochain(
        holochain_path.to_path_buf(),
        path,
        app_port.map(|p| vec![p]).unwrap_or_default(),
        force_admin_port,
      )
    });
  let childs = futures::future::try_join_all(jhs).await?;

  Ok(childs)
}

// Copied over from hc_sandbox (https://github.com/holochain/holochain/blob/540c2497f778cc004c1e7114662733fe197790cc/crates/hc_sandbox/src/run.rs#L32)
// to make it possible to listen to when conductors are ready
pub async fn run(
  holochain_path: &Path,
  sandbox_path: PathBuf,
  app_ports: Vec<u16>,
  force_admin_port: Option<u16>,
) -> anyhow::Result<(Child, Child)> {
  let (port, holochain, lair) =
    run_async(holochain_path, sandbox_path.clone(), force_admin_port).await?;
  println!("Running conductor on admin port {} {:?}", port, app_ports);
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
  let _e = format!("Failed to run holochain at {}", sandbox_path.display());
  Ok((holochain, lair))
}
