use tauri::api::process::Command;

pub fn get_holochain_version() -> String {
  let output = Command::new_sidecar("holochain")
    .or(Err(String::from("Can't find holochain binary")))?
    .args(&["--version"])
    .output()
    .map_err(|err| format!("Failed to get holochain's version: {:?}", err))?;

  output.stdout.split(" ")[1]
}
