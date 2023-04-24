use std::path::PathBuf;

use holochain_manager::versions::HolochainVersion;
use lair_keystore_manager::versions::LairKeystoreVersion;

use crate::launcher::error::LauncherError;

static APP_NAME: &str = "holochain-launcher-0.2";

/// Name of the profile
pub type Profile = String;


/// To store things in different locations during development
fn component_name(name: &str) -> String {
  if cfg!(debug_assertions) {
    format!("{}-dev", name)
  } else {
    String::from(name)
  }
}



/** Config */

/// Path to config files of a given profile
///
/// **Important:** On macOS and Windows this is the same as profile_data_dir().
/// Take care to not accidentally overwrite stuff.
///
/// * **Linux:** `$XDG_CONFIG_HOME/${APP_NAME}/profiles/${profile}` or `$HOME/.config/${APP_NAME}/profiles/${profile}`
/// * **macOS:** `$HOME/Library/Application Support/${APP_NAME}/profiles/${profile}`
/// * **Windows:** `{FOLDERID_RoamingAppData}/${APP_NAME}/profiles/${profile}`
///
/// At the time of writing, ${APP_NAME} = `holochain-launcher`
///
pub fn profile_config_dir(profile: String) -> Result<PathBuf, LauncherError> {
  #[cfg(target_os = "linux")]
  let path = dirs_next::config_dir()
    .ok_or(LauncherError::SystemDirError(String::from("Failed to get profile config dir")))?
    .join(component_name(APP_NAME))
    .join("profiles")
    .join(profile);

  #[cfg(not(target_os = "linux"))]
  let path = profile_data_dir(profile)?.join("config");

  Ok(path)
}

/// Path to config files of holochain versions. Contains `conductor-config.yaml`.
///
/// * **Linux:** `$XDG_CONFIG_HOME/${APP_NAME}/profiles/${profile}/${holochain version}` or `$HOME/.config/${APP_NAME}/profiles/${profile}/${holochain version}`
/// * **macOS:** `$HOME/Library/Application Support/${APP_NAME}/profiles/${profile}/${holochain version}`
/// * **Windows:** `{FOLDERID_RoamingAppData}/${APP_NAME}/profiles/${profile}/${holochain version}`
///
pub fn conductor_config_dir(holochain_version: HolochainVersion, profile: String) -> Result<PathBuf, LauncherError> {
  let version: String = holochain_version.into();
  Ok(profile_config_dir(profile)?.join("holochain").join(version))
}


/// Path to the directory containing the `launcher-config.yaml` of a profile.
///
/// * **Linux:** `$XDG_CONFIG_HOME/${APP_NAME}/profiles/${profile}/launcher` or `$HOME/.config/${APP_NAME}/profiles/${profile}/launcher`
/// * **macOS:** `$HOME/Library/Application Support/${APP_NAME}/profiles/${profile}/config/launcher`
/// * **Windows:** `{FOLDERID_RoamingAppData}/${APP_NAME}/profiles/${profile}/config/launcher`
///
pub fn launcher_config_dir(profile: String) -> Result<PathBuf, LauncherError> {
  Ok(profile_config_dir(profile)?.join("launcher"))
}


/// Path to `launcher-config.yaml` config file of a profile.
///
/// * **Linux:** `$XDG_CONFIG_HOME/${APP_NAME}/profiles/${profile}/launcher/launcher-config.yaml` or `$HOME/.config/${APP_NAME}/profiles/${profile}/launcher/launcher-config.yaml`
/// * **macOS:** `$HOME/Library/Application Support/${APP_NAME}/profiles/${profile}/config/launcher/launcher-config.yaml`
/// * **Windows:** `{FOLDERID_RoamingAppData}/${APP_NAME}/profiles/${profile}/config/launcher/launcher-config.yaml`
///
pub fn launcher_config_path(profile: String) -> Result<PathBuf, LauncherError> {
  Ok(profile_config_dir(profile)?.join("launcher").join("launcher-config.yaml"))
}




/** Logs */

/// Path to log file of a given profile
///
///
/// * **Linux:** `$XDG_CONFIG_HOME/${APP_NAME}/${profile}/logs/launcher.log` or `$HOME/.config/${APP_NAME}/${profile}/logs/launcher.log`
/// * **macOS:** `[`home_dir`]`/Library/Logs/${APP_NAME}/${profile}/launcher.log``
/// * **Windows:** `{FOLDERID_RoamingAppData}/${APP_NAME}/${profile}/logs/launcher.log`
///
/// At the time of writing, ${APP_NAME} = `holochain-launcher`
///
pub fn profile_logs_path(profile: String) -> Result<PathBuf, LauncherError> {
  Ok(profile_logs_dir(profile)?.join("launcher.log"))
}

/// Directory containing logs of a given profile
///
/// **Note:** Log dir is not part of config dir on Linux, such that it can be saved
/// from deletion upon factory reset
///
/// Inspired by here: https://docs.rs/tauri/1.2.3/src/tauri/api/path.rs.html#561
/// But without being bound to the bundle-identifier
///
/// * **Linux:** `$XDG_DATA_HOME/${APP_NAME}/profiles/${profile}/logs` or `$HOME/.local/share/${APP_NAME}/profiles/${profile}/logs`
/// * **macOS:** `[`home_dir`]`/Library/Logs/${APP_NAME}/profiles/${profile}`
/// * **Windows:** `{FOLDERID_RoamingAppData}/${APP_NAME}/profiles/${profile}/logs`
///
/// At the time of writing, ${APP_NAME} = `holochain-launcher`
///
pub fn profile_logs_dir(profile: String) -> Result<PathBuf, LauncherError> {
  #[cfg(target_os = "macos")]
  let path = dirs_next::home_dir()
    .ok_or(LauncherError::SystemDirError(String::from("Failed to get home dir")))?
    .join("Library/Logs")
    .join(component_name(APP_NAME))
    .join("profiles")
    .join(profile);

  #[cfg(not(target_os = "macos"))]
  let path = profile_data_dir(profile)?
    .join("logs");

  Ok(path)
}




/** Data */

/// Path to data of a given profile
///
/// **Important:** On macOS and Windows this is the same as profile_config_dir().
/// Take care to not accidentally overwrite stuff.
///
/// * **Linux:** `$XDG_DATA_HOME/${APP_NAME}/profiles/${profile}` or `$HOME/.local/share/${APP_NAME}/profiles/${profile}`
/// * **macOS:** `$HOME/Library/Application Support/${APP_NAME}/profiles/${profile}`
/// * **Windows:** `{FOLDERID_RoamingAppData}/${APP_NAME}/profiles/${profile}`
///
/// At the time of writing, ${APP_NAME} = `holochain-launcher`
///
pub fn profile_data_dir(profile: String) -> Result<PathBuf, LauncherError> {
  Ok(
    dirs_next::data_dir()
    .ok_or(LauncherError::SystemDirError(String::from("Failed to get profile data dir")))?
    .join(component_name(APP_NAME))
    .join("profiles")
    .join(profile)
  )
}




/** Tauri data */

/// Directory where localStorage of the admin launcher window is stored
///
/// * **Linux:** `$XDG_DATA_HOME/${APP_NAME}/profiles/${profile}/tauri` or `$HOME/.local/share/${APP_NAME}/profiles/${profile}/tauri`
/// * **macOS:** `$HOME/Library/Application Support/${APP_NAME}/profiles/${profile}/tauri`
/// * **Windows:** `{FOLDERID_RoamingAppData}/${APP_NAME}/profiles/${profile}/tauri`
///
/// At the time of writing, ${APP_NAME} = `holochain-launcher`
///
pub fn profile_tauri_dir(profile: String) -> Result<PathBuf, LauncherError> {
  Ok(profile_data_dir(profile)?.join("tauri"))
}




/** Holochain data */

/// Directory where data of different holochain versions is stored
///
/// * **Linux:** `$XDG_DATA_HOME/${APP_NAME}/profiles/${profile}/holochain` or `$HOME/.local/share/${APP_NAME}/profiles/${profile}/holochain`
/// * **macOS:** `$HOME/Library/Application Support/${APP_NAME}/profiles/${profile}/holochain`
/// * **Windows:** `{FOLDERID_RoamingAppData}/${APP_NAME}/profiles/${profile}/holochain`
///
/// At the time of writing, ${APP_NAME} = `holochain-launcher`
///
pub fn profile_holochain_data_dir(profile: String) -> Result<PathBuf, LauncherError> {
  Ok(profile_data_dir(profile)?.join("holochain"))
}

/// Directory where conductor databases and app UI's of a given holochain version are stored
///
/// * **Linux:** `$XDG_DATA_HOME/${APP_NAME}/profiles/${profile}/holochain/${holochain_version}` or `$HOME/.local/share/${APP_NAME}/profiles/${profile}/holochain/${holochain_version}`
/// * **macOS:** `$HOME/Library/Application Support/${APP_NAME}/profiles/${profile}/holochain/${holochain_version}`
/// * **Windows:** `{FOLDERID_RoamingAppData}/${APP_NAME}/profiles/${profile}/holochain/${holochain_version}`
///
/// At the time of writing, ${APP_NAME} = `holochain-launcher`
///
pub fn holochain_version_data_dir(holochain_version: &HolochainVersion, profile: String) -> Result<PathBuf, LauncherError> {
  let minor_version = holochain_version.clone().minor_version();
  Ok(profile_holochain_data_dir(profile)?.join(minor_version))
}




/** Lair data */

/// Directory where data of different lair versions is stored
///
/// * **Linux:** `$XDG_DATA_HOME/${APP_NAME}/profiles/${profile}/lair` or `$HOME/.local/share/${APP_NAME}/profiles/${profile}/lair`
/// * **macOS:** `$HOME/Library/Application Support/${APP_NAME}/profiles/${profile}/lair`
/// * **Windows:** `{FOLDERID_RoamingAppData}/${APP_NAME}/profiles/${profile}/lair`
///
/// At the time of writing, ${APP_NAME} = `holochain-launcher`
///
pub fn profile_lair_dir(profile: String) -> Result<PathBuf, LauncherError> {
  Ok(profile_data_dir(profile)?.join("lair"))
}

/// Directory where data of a given lair version is stored
///
/// * **Linux:** `$XDG_DATA_HOME/${APP_NAME}/profiles/${profile}/lair/${lair_keystore_version}` or `$HOME/.local/share/${APP_NAME}/profiles/${profile}/lair/${lair_keystore_version}`
/// * **macOS:** `$HOME/Library/Application Support/${APP_NAME}/profiles/${profile}/lair/${lair_keystore_version}`
/// * **Windows:** `{FOLDERID_RoamingAppData}/${APP_NAME}/profiles/${profile}/lair/${lair_keystore_version}`
///
/// At the time of writing, ${APP_NAME} = `holochain-launcher`
///
pub fn keystore_data_dir(lair_keystore_version: LairKeystoreVersion, profile: String) -> Result<PathBuf, LauncherError> {
  let version: String = lair_keystore_version.into();
  Ok(profile_lair_dir(profile)?.join(version))
}

