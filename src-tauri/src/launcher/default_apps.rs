use std::collections::HashMap;

use holochain_manager::versions::holochain_types_latest::web_app::WebAppBundle;
use holochain_web_app_manager::WebAppManager;
use holochain_manager::versions::version_manager::VersionManager;


pub async fn install_default_apps_if_necessary(manager: &mut WebAppManager) -> Result<(), String> {
  let apps = manager.list_apps().await?;

  if apps.len() == 0 {
    let dev_hub_bundle = WebAppBundle::decode(include_bytes!("../../../DevHub.webhapp"))
      .or(Err("Malformed Web hApp bundle file"))?;

    let version: String = manager.holochain_manager.version.manager().hdi_version().into();

    log::info!("%*%*%* HDI VERSION: {:?}", version);

    let network_seed = if cfg!(debug_assertions) { Some(String::from("launcher-dev")) } else { None };

    manager
      .install_web_app(
        format!("DevHub-{}", version),
        dev_hub_bundle,
        network_seed,
        HashMap::new(),
        None,
      )
      .await?;
  }

  Ok(())
}