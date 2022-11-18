
use std::path::PathBuf;
use structopt::StructOpt;
use holochain_cli_sandbox::cmds::NetworkCmd;


#[derive(Debug, StructOpt, Clone)]
// This creates a new holochain sandbox
// which is a
// - conductor config
// - databases
// - keystore
pub struct CreateInput {
    /// Number of conductor sandboxes to create.
    #[structopt(short, long, default_value = "1")]
    pub num_sandboxes: usize,

    #[structopt(subcommand)]
    /// Add an optional network config
    pub network: Option<NetworkCmd>,
    /// Set a root directory for conductor sandboxes to be placed into.
    /// Defaults to the system's temp directory.
    /// This directory must already exist.
    #[structopt(long)]
    pub root: Option<PathBuf>,
}