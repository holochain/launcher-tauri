//! Definitions of StructOpt options for use in the CLI

// use holochain_types::prelude::InstalledAppId;
// use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;
use std::io::Read;
use crate::utils;
use crate::error::HcLaunchError;

// const DEFAULT_APP_ID: &str = "test-app";

#[derive(Debug, StructOpt)]
/// Helper for launching holochain apps in a holochain-launcher environment for testing and development purposes.
///
pub struct HcLaunch {
    #[structopt(subcommand)]
    command: HcLaunchSubcommand,
    /// How many agents to run in parallel.
    #[structopt(long, default_value = "1")]
    agents: u32,
}

/// The list of subcommands for `hc launch`
#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::InferSubcommands)]
pub enum HcLaunchSubcommand {
  ///
  /// Launch a .webhapp file in a launcher testing environment.
  WebApp {
    /// Path to .webhapp file to launch.
    path: Option<PathBuf>,

    // #[structopt(long)]
    // /// Port of the UI
    // ui_port: Option<u16>,

    #[structopt(long)]
    /// path to the UI
    ui_path: Option<PathBuf>,

    #[structopt(long, short)]
    watch: bool,

    // todo! add network command
  },
}


impl HcLaunch {
  /// Run this command
  pub async fn run(self) -> anyhow::Result<()> {
    match self.command {
      HcLaunchSubcommand::WebApp {
          path,
          ui_path,
          watch,
        } => {

          match path {
            Some(p) => {
              match p.extension() {
                Some(extension) => {
                  match extension.to_str().unwrap() {
                    "webhapp" => {
                      // unzip the webhapp, prepare UI etc.
                      utils::read_and_prepare_webhapp(&p).await;

                      // generate agents
                      let happ_path = PathBuf::from(".hc_launch/happ.happ");
                      let app_handle = crate::generate_agents(happ_path, self.agents, Some(String::from("mdns")));

                      // launch tauri windows via hc-launch-tauri
                      let tauri_handle = crate::launch_tauri(None, watch);

                      app_handle.join().unwrap();
                      tauri_handle.join().unwrap();
                    }
                    "happ" => {
                      match ui_path {
                        Some(ui_p) => {

                          // check whether path exists
                          if !ui_p.exists() {
                            return Err(anyhow::Error::from(HcLaunchError::UiPathDoesNotExist(format!("{}", ui_p.to_str().unwrap()))));
                          }


                          // generate agents
                          let app_handle = crate::generate_agents(p.clone(), self.agents, Some(String::from("mdns")));

                          // launch tauri windows via hc-launch-tauri
                          let tauri_handle = crate::launch_tauri(Some(ui_p), watch);

                          // println!("Child stdout: {:?}", app_handle.stdout);
                          // // linereader::LineReader::new(app_handle.stdout.unwrap()).for_each(|line| {
                          // //   println!("line: {:?}", std::str::from_utf8(line));
                          // //   Ok(true)
                          // // });
                          // let mut s = [0u8; 500];
                          // let mut stdout = app_handle.stdout.take().expect("Failed to take stdout");

                          // loop {
                          //   stdout.read(&mut s);
                          //   let vec: Vec<u8> = Vec::from(&s as &[u8]);
                          //   println!("Got: {}", String::from_utf8(vec).unwrap());
                          // }

                          app_handle.join().unwrap();
                          // println!("joined app handle");
                          tauri_handle.join().unwrap();
                          println!("joined tauri handle");

                        },
                        None => eprintln!("Error: If you provide a path to a .happ file you also need to specify a path to the UI assets via the --ui-path option.\nRun `hc-launch web-app --help` for help."),
                      }
                    },
                    _ => eprintln!("Error: You need to provide a path that points to either a .webhapp a .happ file."),
                  }
                },
                None => eprintln!("Error: You need to provide a path that points to either a .webhapp or a .happ file.")
              }
            },
            None => println!("You need to provide a path that points to either a .webhapp a .happ file. Auto-detection is not implemented yet.")
          }
        },
    }

    Ok(())
  }
}
