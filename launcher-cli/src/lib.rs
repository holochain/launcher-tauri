use std::env;
use std::fs;
use std::path::PathBuf;

use std::process::Command;
use std::thread::JoinHandle;
use std::process::Stdio;
use std::time::Duration;

// // use lair_keystore_manager::versions::v0_2::LairKeystoreManagerV0_2;
// // use lair_keystore_manager::LairKeystoreManager;
// use lair_keystore_manager::utils::{path_exists, create_dir_if_necessary};

mod utils;
pub mod cli;

pub use cli::HcLaunch;

async fn launch_webhapp(web_happ_path: PathBuf, agents: u32) {

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

  match env::set_current_dir(new_dir) {
    Ok(()) => (),
    Err(e) => panic!("Failed to change current working directory: {:?}", e),
  }

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
    println!("listing happs...");
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


  // println!("output: {:?}", String::from_utf8(output.stdout));

  // let output = if cfg!(target_os = "windows") {
  //   Command::new("cmd")
  //     .args(["/C", "echo hello"])
  //     .output()
  //     .expect("failed to execute process")
  //   } else {
  //     Command::new("sh")
  //       .arg("-c")
  //       .arg("echo hello")
  //       .output()
  //       .expect("failed to execute process")
  //   };

  // let hello = output.stdout;
  // println!("Output: {:?}", String::from_utf8(hello));

  // let _output2 = Command::new("hc")
  //   .args(["s", "call", "-r=$ADMIN_PORT", "install-app-bundle", "--app-id", "test"])
  //   .output()
  //   .expect("failed to execute process");

  // let _output2 = Command::new("hc")
  //   .args(["s", "--piped", "-f=$ADMIN_PORT", "generate", ".launcher-cli/happ.happ", "--run=$HC_PORT", "-a", "test", "network", "mdns"])
  //   .output()
  //   .expect("failed to execute process");


  // extract keystore path from generated .hc files




  // let keystore_path = PathBuf::from(".keystore-tmp");

  // if !path_exists(&keystore_path) {
  //   println!("Initializing keystore.");
  //   match LairKeystoreManagerV0_2::initialize(keystore_path.clone(), String::from("dummypass"))
  //     .await {
  //       Ok(()) => (),
  //       Err(e) => panic!("Error initializing the keystore: {:?}", e),
  //   }
  // } else {
  //   println!("Keystore already initialized. To create a keystore from scratch, delete the .keystore-tmp file.")
  // }

  // println!("Launching keystore...");
  // let lair_keystore_manager =  match LairKeystoreManagerV0_2::launch(log::Level::Warn, keystore_path, String::from("dummypass"))
  //   .await {
  //     Ok(manager) => manager,
  //     Err(e) => panic!("Error launching the keystore: {:?}", e),
  // };



  // println!("Successfully launched the keystore!");


  // receives .webhapp file

  // splits it in UI and .happ, and creates temporary folders for the two

  // run hc sandbox with the .happ file

  // extract the keystore path from the generated .hc files

  // create lair client to sign zome calls with

  // store admin and app websocket ports somewhere

  // pick up app and admin websocket ports and create tauri window that serves from temporary UI folder




  // hc launch xyz.webhapp
  // hc launch --ui-port 8080 yxz.happ  // for hot reloading





  // boot up lair-keystore with dummy password

  // boot up holochain

  // install webhapp

  // create temporary folder for UI assets?

  // spawns window with WindowBuilder

  //








}







