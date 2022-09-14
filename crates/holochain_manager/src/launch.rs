use log;
use std::{collections::HashMap, path::PathBuf};
use tauri::api::process::{Command, CommandChild, CommandEvent};

use lair_keystore_manager::error::LaunchChildError;

use crate::{errors::LaunchHolochainError, errors::InitializeConductorError, versions::HolochainVersion};


enum LaunchHolochainProcessState {
  Pending,
  InitializeConductorError(InitializeConductorError),
  Success,
}

pub async fn launch_holochain_process(
  log_level: log::Level,
  version: HolochainVersion,
  command: Command,
  conductor_config_path: PathBuf,
  password: String,
) -> Result<CommandChild, LaunchHolochainError> {
  let mut envs = HashMap::new();
  envs.insert(String::from("RUST_LOG"), String::from(log_level.as_str()));
  envs.insert(String::from("WASM_LOG"), String::from(log_level.as_str()));

  let (mut holochain_rx, mut holochain_child) = command
    .args(&[
      "-c",
      conductor_config_path.into_os_string().to_str().unwrap(),
      "-p",
    ])
    .envs(envs)
    .spawn()
    .map_err(|err| {
      LaunchHolochainError::LaunchChildError(LaunchChildError::FailedToExecute(format!("{}", err)))
    })?;


  let mut launch_state = LaunchHolochainProcessState::Pending;


  holochain_child
    .write(password.as_bytes())
    .map_err(|err| LaunchHolochainError::ErrorWritingPassword(format!("{:?}", err)))?;
  holochain_child
    .write("\n".as_bytes())
    .map_err(|err| LaunchHolochainError::ErrorWritingPassword(format!("{:?}", err)))?;

  // this loop will end in still pending when the conductor crashes before being ready
  // read events such as stdout
  while let Some(event) = holochain_rx.recv().await {

    match event.clone() {
      CommandEvent::Stdout(line) => {
        log::info!("[HOLOCHAIN v{}] {}", version, line);
        if line == String::from("Conductor ready.") {
          launch_state = LaunchHolochainProcessState::Success;
          break;
        }
      },
      CommandEvent::Stderr(line) => {

        log::info!("[HOLOCHAIN v{}] {}", version, line);

        if line.contains("FATAL PANIC PanicInfo") {
          launch_state = LaunchHolochainProcessState::InitializeConductorError(
            InitializeConductorError::UnknownFatalPanic(
              String::from("Unknown fatal panic when trying to initialize conductor. See log file for details.")
            )
          );
        }
        if line.contains("Could not initialize Conductor from configuration: InterfaceError(WebsocketError(Io(Os { code: 98, kind: AddrInUse, message: \"Address already in use\" })))") {
          launch_state = LaunchHolochainProcessState::InitializeConductorError(
            InitializeConductorError::AddressAlreadyInUse(
              String::from("Could not initialize Conductor from configuration: InterfaceError(WebsocketError(Io(Os { code: 98, kind: AddrInUse, message: \"Address already in use\" })))")
            )
          );
        }
        if line.contains("DatabaseError(SqliteError(SqliteFailure(Error { code: NotADatabase, extended_code: 26 }, Some(\"file is not a database\"))))") {
          launch_state = LaunchHolochainProcessState::InitializeConductorError(
            InitializeConductorError::SqliteError(
              String::from("DatabaseError(SqliteError(SqliteFailure(Error { code: NotADatabase, extended_code: 26 }, Some(\"file is not a database\"))))")
            )
          );
        }
      },
      _ => {
        log::info!("[HOLOCHAIN v{}] {:?}", version, event);
      },
    };

  };

  log::info!("Launched holochain");



  tauri::async_runtime::spawn(async move {
    // read events such as stdout
    while let Some(event) = holochain_rx.recv().await {
      match event.clone() {
        CommandEvent::Stdout(line) => log::info!("[HOLOCHAIN v{}] {}", version, line),
        CommandEvent::Stderr(line) => log::info!("[HOLOCHAIN v{}] {}", version, line),
        _ => log::info!("[HOLOCHAIN v{}] {:?}", version, event),
      };
    }
  });


  match launch_state {
    LaunchHolochainProcessState::Success => {
      log::info!("LaunchHolochainProcessState::Success");
      Ok(holochain_child)
    },
    LaunchHolochainProcessState::InitializeConductorError(e) => {
      log::info!("LaunchHolochainProcessState::InitializeConductorError");
      Err(LaunchHolochainError::CouldNotInitializeConductor(e))
    },
    LaunchHolochainProcessState::Pending => {
      log::info!("LaunchHolochainProcessState::Pending");
      Err(LaunchHolochainError::ImpossibleError("LaunchHolochainProcessState still pending after launching the holochain process.".into()))
    }
  }

}

