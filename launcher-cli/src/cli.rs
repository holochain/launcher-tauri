//! Definitions of StructOpt options for use in the CLI

use crate::cmds::*;
use holochain_types::prelude::InstalledAppId;
use std::path::Path;
use std::path::PathBuf;
use structopt::StructOpt;

const DEFAULT_APP_ID: &str = "test-app";

#[derive(Debug, StructOpt)]
/// Helper for generating, running, and interacting with Holochain Conductor "sandboxes".
///
/// A sandbox is a directory containing a conductor config, databases, and keystore,
/// with a single Holochain app installed in the conductor:
/// Everything you need to quickly run your app in holochain,
/// or create complex multi-conductor sandboxes for testing.
pub struct HcLaunch {
    #[structopt(subcommand)]
    command: HcLaunchSubcommand,
    /// How many agents to run in parallel. Defaults to 1.
    #[structopt(long, default_value = 1)]
    agents: u32,
}

/// The list of subcommands for `hc sandbox`
#[derive(Debug, StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::InferSubcommands)]
pub enum HcLaunchSubcommand {
    /// Launch a holochain app in a launcher environment.
    ///
    /// Launch from a .webhapp file.
    WebApp {
        #[structopt(short, long)]
        /// Path to the .webhapp file to launch.
        path: String,
    },

    // TODO! add option for hot reloading by serving from localhost
    // Localhost {
    // // Specify a port from which the UI is being served to allow hot-reloading.
    // #[structopt(short, long)]
    // port: u32,
    // }
}


impl HcLaunch {
    /// Run this command
    pub async fn run(self) -> anyhow::Result<()> {
        match self.command {
          HcLaunchSubcommand::WebApp {
                path
            } => {
                // extract webhapp and run it in tauri window(s)
              println!("This would run the webhapp at path {} for {} agents", path, self.agents);
            }
        }

        Ok(())
    }
}

async fn run_n(
    holochain_path: &Path,
    paths: Vec<PathBuf>,
    app_ports: Vec<u16>,
    force_admin_ports: Vec<u16>,
) -> anyhow::Result<()> {
    let run_holochain = |holochain_path: PathBuf, path: PathBuf, ports, force_admin_port| async move {
        crate::run::run(&holochain_path, path, ports, force_admin_port).await?;
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

async fn generate(
    holochain_path: &Path,
    happ: Option<PathBuf>,
    create: Create,
    app_id: InstalledAppId,
) -> anyhow::Result<Vec<PathBuf>> {
    let happ = crate::bundles::parse_happ(happ)?;
    let paths = crate::sandbox::default_n(holochain_path, create, happ, app_id).await?;
    crate::save::save(std::env::current_dir()?, paths.clone())?;
    Ok(paths)
}
