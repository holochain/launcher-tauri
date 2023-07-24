//! Definitions of StructOpt options for use in the CLI

// use holochain_types::prelude::InstalledAppId;
// use std::path::Path;
use clap::Parser;
use holochain_cli_sandbox::CmdRunner;
use holochain_cli_sandbox::calls::{InstallApp, Call, AdminRequestCli, attach_app_interface, AddAppWs};
use holochain_cli_sandbox::cli::generate;
use holochain_cli_sandbox::run::run_async;
use holochain_launcher_utils::window_builder::UISource;
use holochain_types::prelude::InstalledAppId;
use holochain_trace::Output;
use tokio::process::Child;
use std::path::{PathBuf, Path};

use crate::launch_tauri::launch_tauri;
use crate::prepare_webapp;
use holochain_cli_sandbox::cmds::{Create, Existing, NetworkCmd, NetworkType};


#[derive(Debug, Parser)]
/// Helper for launching holochain apps in a Holochain Launcher environment for testing and development purposes.
///
pub struct HcLaunch {
  /// Instead of the normal "interactive" passphrase mode,
  /// collect the passphrase by reading stdin to the end.
  #[arg(long)]
  piped: bool,

  /// Set the path to the holochain binary.
  #[arg(long, env = "HC_HOLOCHAIN_PATH", default_value = "holochain")]
  holochain_path: PathBuf,

  /// Path to .webhapp or .happ file to launch. If a .happ file is passed, either
  /// a UI path must be specified via --ui-path or a port pointing to a localhost
  /// server via --ui-port.
  path: Option<PathBuf>,


  /// Install and run the app into already running conductors.
  /// The number of sandboxes cannot be specified if this flag is used
  /// since the app will just be installed into all existing conductors.
  #[arg(long)]
  reuse_conductors: bool,

  /// Install the app with a specific network seed.
  /// This option can currently only be used with the `--reuse-conductors` flag.
  #[arg(long)]
  network_seed: Option<String>,

  /// Install the app with a specific app id. By default the app id is derived
  /// from the name of the .webhapp/.happ file that you pass but this option allows
  /// you to set it explicitly.
  #[arg(long)]
  app_id: Option<String>,


  #[arg(long)]
  /// Port pointing to a localhost server that serves your assets.
  /// NOTE: This is only meant for development purposes! Apps can behave differently when
  /// served from a localhost server than when actually running in the Holochain Launcher.
  /// Use the --ui-path flag pointing to your built and bundled files instead or directly pass
  /// the packaged .webhapp to test the actual behavior of your hApp in the Holochain Launcher.
  ui_port: Option<u16>,

  #[arg(long)]
  /// path to the UI. Required if a .happ file is passed.
  ui_path: Option<PathBuf>,

  /// Watch for file changes in the UI folder. Requires --ui-path to be specified.
  #[arg(long, short)]
  watch: bool,

  /// (flattened)
  #[command(flatten)]
  create: Create,

  /// Explicitly allow to use the official production signaling and/or bootstrap server(s)
  /// NOTE: It is strongly recommended to use local signaling and bootstrap servers during development.
  /// Bootstrap and signaling server for development can be started via `hc run-local-services`.
  #[arg(long)]
  force_production: bool,
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

    match (self.reuse_conductors, self.create.num_sandboxes != 1) {
      (true, true) => {
        eprintln!("[hc launch] WARNING: If you pass the --reuse-conductors flag the -n (--num-sandboxes) argument will be ignored.");
      },
      _ => (),
    }

    match (self.reuse_conductors, self.network_seed.clone()) {
      (false, Some(_seed)) => {
        eprintln!("[hc launch] ERROR: The --network-seed option can currently only be taken into account when installing the app to already running conductors with the --reuse-conductors flag.");
        panic!("ERROR: The --network-seed option can currently only be taken into account when installing the app to already running conductors with the --reuse-conductors flag.");
      },
      _ => (),
    }

    match self.create.in_process_lair {
      true => {
        eprintln!("[hc launch] ERROR: The --in-process-lair flag is only supported by hc sandbox but not by hc launch.");
        panic!("ERROR: The --in-process-lair flag is only supported by hc sandbox but not by hc launch.");
      },
      _ => (),
    }

    if let Some(_port) = self.ui_port {
      println!("\n[hc launch] ------ WARNING ------");
      println!(r#"[hc launch] You are running hc launch pointing to a localhost server. This is meant for development purposes
[hc launch] only as apps can behave differently than when actually running in the Holochain Launcher.
[hc launch] To test the real behavior, use --ui-path instead and point to a folder with your built and bundled files
[hc launch] or pass an already packaged .webhapp as an argument."#);
      println!("[hc launch] ---------------------\n");

    }

    // Fail if production signaling server is used unless the --force-production flag is used
    if let Some(NetworkCmd::Network(n)) = self.create.clone().network {
      match n.transport {
        NetworkType::WebRTC { signal_url: s } => {
          if (s == String::from("ws://signal.holo.host") || s == String::from("wss://signal.holo.host")) && self.force_production == false {
            eprintln!(r#"
ERROR

You are attempting to use the official production signaling server of holochain.
It is recommended to instead use the `hc run-local-services` command of the holochain CLI to spawn a local bootstrap and signaling server for testing.
If you are sure that you want to use the production signaling server with hc launch, use the --force-production flag.

"#);

            panic!("Attempted to use production signaling server without explicitly allowing it.");
          }
        },
        _ => ()
      }

      match n.bootstrap {
        Some(url) => {
          if (url.to_string() == "https://bootstrap.holo.host") || (url.to_string() == "http://bootstrap.holo.host") && self.force_production == false {
            eprintln!(r#"
ERROR

You are attempting to use the official production bootstrap server of holochain.
It is recommended to instead use the `hc run-local-services` command of the holochain CLI to spawn a local bootstrap and signaling server for testing.
If you are sure that you want to use the production bootstrap server with hc launch, use the --force-production flag.

"#);

            panic!("Attempted to use production bootstrap server without explicitly allowing it.");
          }
        },
        _ => (),
      }
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

                // extraxt filename of .webhapp if required
                let app_id = match self.app_id.clone() {
                  Some(id) => id,
                  None => p.as_path().file_stem().unwrap().to_str().unwrap().to_string(),
                };

                let happ_file_name = format!("{}.happ", p.as_path().file_stem().unwrap().to_str().unwrap());

                let happ_path = temp_folder.join(happ_file_name);

                if self.reuse_conductors {

                  // read the .hc file to get the existing sandbox directories
                  let pwd = std::env::current_dir().unwrap();
                  let dot_hc_path = pwd.join(".hc");

                  while !dot_hc_path.exists() {
                    println!("[hc launch with --reuse-conductors] No existing sandbox conductors found (yet). Waiting for sandboxes...");
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                  }

                  let dot_hc_content = match std::fs::read_to_string(dot_hc_path) {
                    Ok(p) => p,
                    Err(e) => {
                      println!("[hc launch] ERROR: Failed to read content of .hc file: {}", e);
                      panic!("Failed to read content of .hc file: {}", e);
                    }
                  };

                  let existing_paths = dot_hc_content
                    .lines()
                    .map(|path_str| PathBuf::from(path_str))
                    .collect::<Vec<PathBuf>>();


                  let running_ports = get_running_ports(pwd, existing_paths.len());

                  let install_app = InstallApp {
                    app_id: Some(String::from(app_id.clone())),
                    agent_key: None,
                    path: happ_path,
                    network_seed: self.network_seed,
                  };

                  let call = Call {
                    running: running_ports,
                    existing: Existing {
                      existing_paths: vec![],
                      all: true,
                      last: false,
                      indices: vec![],
                    },
                    call: AdminRequestCli::InstallApp(install_app),
                  };

                  holochain_cli_sandbox::calls::call(&self.holochain_path, call, Output::Log).await?;

                } else {
                  // clean existing sandboxes
                  holochain_cli_sandbox::save::clean(std::env::current_dir()?, Vec::new())?;

                  // spawn sandboxes
                  println!("[hc launch] Spawning sandbox conductors.");
                  let child_processes = spawn_sandboxes(
                    &self.holochain_path,
                    happ_path,
                    self.create,
                    app_id.to_string(),
                  ).await?;

                  tauri::async_runtime::spawn(async move {
                    tokio::signal::ctrl_c().await.unwrap();
                    holochain_cli_sandbox::save::release_ports(std::env::current_dir().unwrap()).await.unwrap();
                    println!("Released ports.");
                    temp_dir.close().unwrap();
                    // killing child processes
                    for (mut holochain_process, lair_process) in child_processes {
                      holochain_process.start_kill().unwrap();
                      if let Some(mut p) = lair_process {
                        p.start_kill().unwrap();
                      }
                    }
                    println!("Killed holochain processes, press Ctrl+C to quit.");
                    std::process::exit(0);
                  });
                }

                let passphrase = holochain_util::pw::pw_get()?;

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
                  }
                }

                let local_storage_path = temp_folder.join("tauri");

                println!("[hc launch] Launching tauri windows.");
                launch_tauri(ui_source, app_id.to_string(), local_storage_path, self.watch, passphrase);
              }
              "happ" => {
                match maybe_ui_source {
                  Some(ui_source) => {

                    // extraxt filename of .happ if required
                    let app_id = match self.app_id.clone() {
                      Some(id) => id,
                      None => p.as_path().file_stem().unwrap().to_str().unwrap().to_string(),
                    };

                    if self.reuse_conductors {

                      // read the .hc file to get the existing sandbox directories
                      let pwd = std::env::current_dir().unwrap();
                      let dot_hc_path = pwd.join(".hc");

                      while !dot_hc_path.exists() {
                        println!("[hc launch with --reuse-conductors] No existing sandbox conductors found (yet). Waiting for sandboxes...");
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                      }

                      let dot_hc_content = match std::fs::read_to_string(dot_hc_path) {
                        Ok(p) => p,
                        Err(e) => {
                          println!("[hc launch] ERROR: Failed to read content of .hc file: {}", e);
                          panic!("Failed to read content of .hc file: {}", e);
                        }
                      };

                      let existing_paths = dot_hc_content
                        .lines()
                        .map(|path_str| PathBuf::from(path_str))
                        .collect::<Vec<PathBuf>>();

                      let running_ports = get_running_ports(pwd, existing_paths.len());

                      let install_app = InstallApp {
                        app_id: Some(String::from(app_id.clone())),
                        agent_key: None,
                        path: p,
                        network_seed: self.network_seed,
                      };

                      let call = Call {
                        running: running_ports,
                        existing: Existing {
                          existing_paths: vec![],
                          all: true,
                          last: false,
                          indices: vec![],
                        },
                        call: AdminRequestCli::InstallApp(install_app),
                      };

                      holochain_cli_sandbox::calls::call(&self.holochain_path, call, Output::Log).await?;

                    } else {
                      // clean existing sandboxes
                      holochain_cli_sandbox::save::clean(std::env::current_dir()?, Vec::new())?;

                      // spawn sandboxes
                      println!("[hc launch] Spawning sandbox conductors.");
                      let child_processes = spawn_sandboxes(
                        &self.holochain_path,
                        p,
                        self.create,
                        app_id.clone(),
                      ).await?;


                      tauri::async_runtime::spawn(async move {
                        tokio::signal::ctrl_c().await.unwrap();
                        holochain_cli_sandbox::save::release_ports(std::env::current_dir().unwrap()).await.unwrap();
                        println!("Released ports.");
                        // killing child processes
                        for (mut holochain_process, lair_process) in child_processes {
                          holochain_process.start_kill().unwrap();
                          if let Some(mut p) = lair_process {
                            p.start_kill().unwrap();
                          }
                        }
                        println!("Killed holochain processes, press Ctrl+C to quit.");
                        std::process::exit(0);
                      });

                    }

                    let passphrase = holochain_util::pw::pw_get()?;


                    // In case a ui path is passed, check whether it exists, otherwise wait
                    if let Some(ui_p) = self.ui_path {
                      while !ui_p.exists() {
                        println!("[hc launch] Specified UI path does not exist (yet). Waiting before launching tauri windows...");
                        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
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
) -> anyhow::Result<Vec<(Child, Option<Child>)>> {
  let sandbox_paths = generate(holochain_path, Some(happ_path), create, app_id, Output::Log).await?;

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

// copied over from hc_sanbox because it's not public (https://github.com/holochain/holochain/blob/03f315be92991f374cba341d210340f7e1141578/crates/hc_sandbox/src/cli.rs#L190)
async fn run_n(
  holochain_path: &Path,
  paths: Vec<PathBuf>,
  app_ports: Vec<u16>,
  force_admin_ports: Vec<u16>,
) -> anyhow::Result<Vec<(Child, Option<Child>)>> {
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

// // Copied over from hc_sandbox (https://github.com/holochain/holochain/blob/540c2497f778cc004c1e7114662733fe197790cc/crates/hc_sandbox/src/run.rs#L32)
// // to make it possible to listen to when conductors are ready
pub async fn run(
  holochain_path: &Path,
  sandbox_path: PathBuf,
  app_ports: Vec<u16>,
  force_admin_port: Option<u16>,
) -> anyhow::Result<(Child, Option<Child>)> {
  let (port, holochain, lair) =
    run_async(holochain_path, sandbox_path.clone(), force_admin_port, Output::Log).await?;
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


/// Reads the contents of the .hc_live_{n} files in the given path where n is 0 to n_expected
pub fn get_running_ports(path: PathBuf, n_expected: usize) -> Vec<u16> {
  let mut running_ports = Vec::new();

  // get ports of running conductors from .hc_live and if there are none, throw an error.
  for n in 0..n_expected {
    let dot_hc_live_path = path.join(format!(".hc_live_{}", n));

    while !dot_hc_live_path.exists() {
      println!("[hc launch with --reuse-conductors] No *running* sandbox conductor found (yet). Waiting for running sandbox conductor(s)...");
      std::thread::sleep(std::time::Duration::from_secs(2));
    }

    let admin_port = match std::fs::read_to_string(dot_hc_live_path) {
      Ok(p) => p,
      Err(e) => {
        match n {
          0 => {
            println!("[hc launch] ERROR: No running sandbox conductors found. If you use the --reuse-conductors flag there need to be existing sandbox conductors running.\n {}", e);
            panic!("ERROR: No running snadbox conductors found. If you use the --reuse-conductors flag there need to be existing sandbox conductors running.\n {}", e);
          },
          _ => {
            println!("[hc launch] ERROR: Not enough running sandbox conductors found. If you use the --reuse-conductors flag there need to be as many running sandbox conductors as mentioned in the .hc file.\n {}", e);
            panic!("ERROR: No running snadbox conductors found. If you use the --reuse-conductors flag there need to be as many running sandbox conductors as mentioned in the .hc file.\n {}", e);
          }
        }
      }
    };

    let admin_port = match admin_port.trim().parse::<u16>() {
      Ok(u) => u,
      Err(e) => {
        println!("[hc launch] ERROR: Failed to convert admin port from String to u16: {}", e);
        panic!("Failed to convert admin port from String to u16: {}", e);
      }
    };

    running_ports.push(admin_port);
  }

  running_ports

}

