use std::env;
use std::path::PathBuf;

use std::process::Command;
use std::thread::JoinHandle;
use std::time::Duration;

mod utils;
pub mod cli;

pub use cli::HcLaunch;

async fn launch_webhapp(web_happ_path: PathBuf, agents: u32) -> anyhow::Result<()> {

  println!("web_happ_path: {:?}", web_happ_path);

  utils::read_and_prepare_webhapp(web_happ_path).await;

  // let maybe_bytes = fs::read(&web_happ_path).or(Err("Failed to read Web hApp bundle file"));



  // Running a sandbox: hc s clean && hc s call -r=$ADMIN_PORT install-app-bundle --app-id
  // RUST_LOG=warn WASM_LOG=debug echo \"pass\" | hc s --piped -f=$ADMIN_PORT generate ./workdir/we.happ --run=$HC_PORT -a we network mdns

  println!("cleaning sandboxes...");
  let _output = Command::new("hc")
    .args(["s", "clean"])
    .output()
    .expect("failed to execute process");

  // create a new random id to identify the sandboxes and be able to retrieve the directory to the lair-keystores
  let sandbox_identifier = nanoid::nanoid!();

  // remove existing .hc_launch file if present
  let _ = std::fs::remove_file(".hc_launch");


  // pass dummy lair password
  Command::new("echo")
    .args(["pass", "|"])
    .output()
    .expect("failed to execute process");

  let mut app_handles: Vec<JoinHandle<()>> = vec![];

  for agent in 0..agents {
    // let app_handle = utils::spawn_agent_app_instance(format!("Agent-{}", agent), Some(sandbox_identifier.clone()), false);
    // set sandbox_identifier to None for now since conductors seem to not communicate with each other otherwise
    let app_handle = utils::spawn_agent_app_instance(format!("Agent-{}", agent), None, false);

    app_handles.push(app_handle);
    std::thread::sleep(Duration::from_millis(100));
  }



  println!("Current path");
  let output_pwd = Command::new("pwd")
    .output()
    .expect("failed to execute process");
  println!("output pwd: {:?}", String::from_utf8(output_pwd.stdout));

  // current working directory:
  let current_dir = env::current_dir();
  let new_dir = match current_dir {
    Ok(dir) => dir.join("tauri").join("src-tauri"),
    Err(e) => panic!("Failed at getting the current working directory: {:?}", e),
  };

  env::set_current_dir(new_dir)?;

  println!("Getting new pwd");
  let output_pwd = Command::new("pwd")
    .output()
    .expect("failed to execute process");
  println!("new pwd: {:?}", String::from_utf8(output_pwd.stdout));





  // println!("launching tauri application");
  // let _output6 = Command::new("cd")
  //   .args(["../tauri/src-tauri", "&&", "cargo", "tauri", "dev"])
  //   .output()
  //   .expect("failed to execute process");

  let tauri_dev_handle = std::thread::spawn(|| {
    println!("cargo tauri dev sleeps for 15 seconds.");

    std::thread::sleep(Duration::from_millis(15000));

    // make sure that happ is actually installed
    let _output4 = Command::new("echo")
      .args(["pass", "|"])
      .output()
      .expect("failed to execute process");


    println!("#*#*# cargo tauri dev #*#*#");
    let output7 = Command::new("cargo")
      .args(["tauri", "dev"])
      .output()
      .expect("failed to execute process");

    println!("cargo tauri dev output: {:?}", String::from_utf8(output7.stdout));
  });



  println!("done!");

  for handle in app_handles {
    handle.join().unwrap();
  }

  tauri_dev_handle.join().unwrap();
  println!("exited sandbox!");

  Ok(())

}




  // receives .webhapp file

  // splits it in UI and .happ, and creates temporary folders for the two

  // run hc sandbox with the .happ file

  // extract the keystore path from the generated .hc files

  // create lair client to sign zome calls with

  // store admin and app websocket ports somewhere

  // pick up app and admin websocket ports and create tauri window that serves from temporary UI folder




