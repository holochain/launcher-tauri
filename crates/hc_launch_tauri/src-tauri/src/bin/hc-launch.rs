
use structopt::StructOpt;


fn main() {
  if std::env::var_os("RUST_LOG").is_some() {
      observability::init_fmt(observability::Output::Log).ok();
  }
  let ops = holochain_cli_launch::cli::HcLaunch::from_args();
  tauri::async_runtime::block_on( async {
    ops.run().await
  }).expect("Failed to run HcLaunch in tauri runtime.");
}

