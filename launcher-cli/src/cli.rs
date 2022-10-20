//! Definitions of StructOpt options for use in the CLI

// use holochain_types::prelude::InstalledAppId;
// use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

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
  Webhapp {
    /// Path to .webhapp file to launch.
    path: Option<PathBuf>,
  },
  /// Launch a .happ file in a launcher testing environment while pointing to a UI running on localhost.
  Localhost {
    #[structopt(short, long)]
    /// Path to the .happ file to launch.
    path: String,

    #[structopt(long)]
    /// Port of the UI
    port: u16,
  }
}


impl HcLaunch {
  /// Run this command
  pub async fn run(self) -> anyhow::Result<()> {
    match self.command {
      HcLaunchSubcommand::Webhapp {
          path
        } => {
          // extract webhapp and run it in tauri window(s)
          // println!("This would run the webhapp at path {} for {} agent(s).", path, self.agents);
          match path {
            Some(p) => crate::launch_webhapp(p, self.agents).await?,
            None => println!("You need to provide a .webhapp path. Auto-detection is not implemented yet.")
          }
        },
      HcLaunchSubcommand::Localhost {
          path,
          port
        } => {
          // extract webhapp and run it in tauri window(s)
          println!("Not implemented yet. But this would run a .happ file at path {} and the corresponding UI at port {} for {} agent(s).", path, port, self.agents);
        },
    }

    Ok(())
  }
}
