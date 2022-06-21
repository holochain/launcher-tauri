use std::collections::HashMap;

use holochain_manager::versions::holochain_types_latest::web_app::WebAppBundle;
use holochain_web_app_manager::WebAppManager;

pub async fn install_default_apps_if_necessary(manager: &mut WebAppManager) -> Result<(), String> {
  let apps = manager.list_apps().await?;

  if apps.len() == 0 {
    let dev_hub_bundle = WebAppBundle::decode(include_bytes!("../../../DevHub.webhapp"))
      .or(Err("Malformed Web hApp bundle file"))?;

    let version: String = manager.holochain_manager.version.into();

    let uid = if cfg!(debug_assertions) { None } else { None };

    manager
      .install_web_app(
        format!("DevHub-{}", version),
        dev_hub_bundle,
        uid,
        HashMap::new(),
        None,
      )
      .await?;
  }

  Ok(())
}