use structopt::StructOpt;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  println!("hc launch launched.");
  if std::env::var_os("RUST_LOG").is_some() {
    observability::init_fmt(observability::Output::Log).ok();
  }
  let opt = holochain_cli_launch::cli::HcLaunch::from_args();
  opt.run().await
}

