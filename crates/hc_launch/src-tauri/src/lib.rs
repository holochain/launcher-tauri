//! A CLI to launch holochain apps in a Holochain Launcher environment for testing and development purposes.
//!
//! If you find a bug or have a feature request, please open an [issue](https://github.com/holochain/launcher/issues)
//! or make a PR.
//!
//!
//! # Using the hc launch CLI tool:
//!
//! **Note:**
//! If you use the cli tool inside holonix, all the commands below will be available as `hc launch`
//! instead of `hc-launch`.
//!
//!
//! ## Example Usage
//!
//! * Launch a .webhapp with 2 agents communicating over mdns network and initializing lair-keystore "on-the-fly"
//! by piping a password through on the command line:
//! ```bash
//! echo pass | hc-launch --piped -n 2 path/to/my/app.webhapp network mdns
//! ```
//! <br>
//!
//! * Launch a .happ with the UI assets specified with the `--ui-path` option for 2 agents communicating over mdns network, initializing lair-keystore "on-the-fly"
//! by piping a password through on the command line and watching for file changes in the specified UI path:
//! ```bash
//! echo pass | hc-launch --piped -n 2 path/to/my/app.happ --ui-path path/to/my/ui/assets --watch network mdns
//! ```
//! <br>
//!
//! * Launch a .happ while pointing to a port on localhost from where you serve the UI assets:<br>
//! **Note:** This mode is only meant for development purposes! Serving assets from localhost will result in different
//! behavior than running the app in the real Holochain Launcher.
//! ```bash
//! echo pass | hc-launch --piped -n 2 path/to/my/app.happ --ui-port 5500 network mdns
//! ```
//! <br>
//!
//! * Show the help section with all the available commands and options:
//! ```bash
//! hc-launch --help
//! ```
//!
//! ## Available Commands
//!
//! ```text
//! USAGE:
//!    hc-launch [FLAGS] [OPTIONS] [path] [SUBCOMMAND]
//!
//! FLAGS:
//!    -h, --help
//!            Prints help information
//!
//!        --piped
//!            Instead of the normal "interactive" passphrase mode, collect the passphrase by reading stdin to the end
//!
//!    -V, --version
//!            Prints version information
//!
//!    -w, --watch
//!            Watch for file changes in the UI folder. Requires --ui-path to be specified
//!
//!
//! OPTIONS:
//!    -d, --directories <directories>...
//!            Specify the directory name for each sandbox that is created. By default, new sandbox directories get a
//!            random name like "kAOXQlilEtJKlTM_W403b". Use this option to override those names with something explicit.
//!
//!            For example `hc gen -r path/to/my/chains -n 3 -d=first,second,third` will create three sandboxes with
//!            directories named "first", "second", and "third".
//!        --holochain-path <holochain-path>
//!            Set the path to the holochain binary [env: HC_HOLOCHAIN_PATH=]  [default: holochain]
//!
//!    -n, --num-sandboxes <num-sandboxes>
//!            Number of conductor sandboxes to create [default: 1]
//!
//!        --root <root>
//!            Set a root directory for conductor sandboxes to be placed into. Defaults to the system's temp directory.
//!            This directory must already exist
//!
//!        --ui-path <ui-path>
//!            path to the UI. Required if a .happ file is passed
//!
//!        --ui-port <ui-port>
//!            Port pointing to a localhost server that serves your assets. NOTE: This is only meant for development
//!            purposes! Apps can behave differently when served from a localhost server than when actually running in the
//!            Holochain Launcher. Use the --ui-path flag pointing to your built and bundled files instead or directly pass
//!            the packaged .webhapp to test the actual behavior of your hApp in the Holochain Launcher
//!
//! ARGS:
//!    <path>
//!            Path to .webhapp or .happ file to launch. If a .happ file is passed, either a UI path must be specified via
//!            --ui-path or a port pointing to a localhost server via --ui-port
//!
//! SUBCOMMANDS:
//!    help       Prints this message or the help of the given subcommand(s)
//!    network
//!
//!
//! ```
//!
//!
//!

pub mod commands;
pub mod cli;
pub mod error;
pub mod launch_tauri;
pub mod prepare_webapp;
