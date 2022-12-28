use structopt::StructOpt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  if std::env::var_os("WEBKIT_DISABLE_COMPOSITING_MODE").is_none() {
    panic!("WEBKIT_DISABLE_COMPOSITING_MODE is not set!");
  }
  if std::env::var_os("GIO_MODULE_DIR").is_none() {
    panic!("GIO_MODULE_DIR is not set!");
  }
  if std::env::var_os("RUST_LOG").is_some() {
    observability::init_fmt(observability::Output::Log).ok();
  }
  let opt = holochain_cli_launch::cli::HcLaunch::from_args();
  opt.run().await
}

