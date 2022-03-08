use reqwest::Url;

use crate::{setup::config::plugins_folder_path, state::LauncherState};
use data_encoding::HEXLOWER;
use minisign_verify::{PublicKey, Signature};
use ring::digest::{Context, Digest, SHA256};
use std::fs;
use std::fs::File;
use std::io::copy;
use std::io::Cursor;
use std::io::{BufReader, Read};
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

const TRUSTED_PUBLIC_KEY: &str = "RWQ0soWur437hgsb623qnmwjlzvTP4AzCLtOC1xYsMUb472NSWMqEfUa";

#[tauri::command]
pub async fn execute_plugin_install(
  _state: tauri::State<'_, LauncherState>,
  binary_url: String,
  hash_url: String,
  signature_url: String,
) -> Result<(), String> {
  log::info!("Installing plugin: {}", binary_url);

  download_file(binary_url.clone(), true).await?;
  download_file(hash_url.clone(), false).await?;
  download_file(signature_url.clone(), false).await?;

  // TODO if check failed, deleted files
  check_hash(binary_url.clone(), hash_url.clone())?;
  check_signature(hash_url, signature_url)?;

  log::info!("Installed plugin successfully");

  Ok(())
}

async fn download_file(url: String, is_executable: bool) -> Result<(), String> {
  let url = Url::parse(&url).map_err(|err| err.to_string())?;

  let fname = url
    .path_segments()
    .and_then(|segments| segments.last())
    .and_then(|name| if name.is_empty() { None } else { Some(name) })
    .ok_or(String::from("Error install plugin: no name provided."))?;
  log::info!("File to download: '{}'", fname);

  let plugins_folder = plugins_folder_path();
  let file_path = plugins_folder.join(fname);
  log::info!("File will be located under: '{:?}'", file_path);
  let mut dest = File::create(file_path.clone()).map_err(|err| {
    log::error!("Error install plugin: {}", err);
    err.to_string()
  })?;

  let response = reqwest::get(url.clone())
    .await
    .map_err(|err| err.to_string())?;
  let mut content = Cursor::new(response.bytes().await.map_err(|err| err.to_string())?);
  copy(&mut content, &mut dest).map_err(|err| err.to_string())?;

  if is_executable {
    fs::set_permissions(file_path, fs::Permissions::from_mode(0o755))
      .map_err(|err| err.to_string())?;
  }

  Ok(())
}

fn check_hash(binary_url: String, hash_url: String) -> Result<(), String> {
  let binary_file_path = parse_file_path(binary_url)?;
  let hash_file_path = parse_file_path(hash_url)?;

  let input = File::open(binary_file_path)
    .map_err(|e| e.to_string())?;
  let reader = BufReader::new(input);
  let digest = sha256_digest(reader)?;
  let encoded_digest = HEXLOWER.encode(digest.as_ref());
  log::info!("SHA-256 digest is {}", encoded_digest);

  let hash_content = fs::read_to_string(hash_file_path).map_err(|e| e.to_string())?;
  if !hash_content.contains(&encoded_digest) {
    return Err("Hash of the file doesn't match".to_string())
  }

  Ok(())
}

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, String> {
  let mut context = Context::new(&SHA256);
  let mut buffer = [0; 1024];

  loop {
    let count = reader.read(&mut buffer).map_err(|e| e.to_string())?;
    if count == 0 {
      break;
    }
    context.update(&buffer[..count]);
  }

  Ok(context.finish())
}

fn check_signature(hash_url: String, signature_url: String) -> Result<(), String> {
  let public_key = PublicKey::from_base64(TRUSTED_PUBLIC_KEY).map_err(|e| e.to_string())?;

  let hash_file_path = parse_file_path(hash_url)?;
  let signature_file_path = parse_file_path(signature_url)?;

  let source_content = fs::read_to_string(hash_file_path).map_err(|e| e.to_string())?;
  let signed_content = fs::read_to_string(signature_file_path).map_err(|e| e.to_string())?;

  let signature = Signature::decode(&signed_content).map_err(|e| e.to_string())?;

  public_key
    .verify(source_content.as_bytes(), &signature, false)
    .map_err(|e| e.to_string())?;

  Ok(())
}

fn parse_file_path(url: String) -> Result<PathBuf, String> {
  let url = Url::parse(&url).map_err(|err| err.to_string())?;

  let fname = url
    .path_segments()
    .and_then(|segments| segments.last())
    .and_then(|name| if name.is_empty() { None } else { Some(name) })
    .ok_or(String::from("Error install plugin: no name provided."))?;

  let plugins_folder = plugins_folder_path();
  let file_path = plugins_folder.join(fname);

  Ok(file_path)
}
