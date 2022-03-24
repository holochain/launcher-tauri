use std::{collections::HashMap, fs, path::PathBuf};

use holochain_manager::versions::{utils::create_dir_if_necessary, HolochainVersion};
use holochain_web_app_manager::ManagedApp;

use super::utils::{build_caddyfile_contents, launch_caddy_process, reload_caddy};

pub struct WebAppConfig {
  pub app_ui_port: u16,
  pub path_to_web_app: PathBuf,
}

pub struct HolochainWebAppsSummary {
  running_apps: HashMap<String, ManagedApp>,
  conductor_admin_port: u16,
  conductor_app_interface_port: u16,
}

pub struct HolochainWebAppsSummaryWithPort {
  running_apps: HashMap<String, WebAppConfig>,
  conductor_admin_port: u16,
  conductor_app_interface_port: u16,
}

pub type RunningWebApps = HashMap<HolochainVersion, HolochainWebAppsSummaryWithPort>;

pub struct CaddyManager {
  environment_path: PathBuf,
  caddy_admin_port: u16,
  running_apps: RunningWebApps,
}

impl CaddyManager {
  pub fn launch(environment_path: PathBuf) -> Result<Self, String> {
    let caddy_admin_port = portpicker::pick_unused_port().expect("No ports free");

    create_dir_if_necessary(&environment_path);

    launch_caddy_process(caddyfile_path(environment_path.clone()))?;

    Ok(CaddyManager {
      environment_path,
      running_apps: HashMap::new(),
      caddy_admin_port,
    })
  }

  pub fn update_running_apps(
    &mut self,
    summaries: HashMap<HolochainVersion, HolochainWebAppsSummary>,
  ) -> Result<(), String> {
    let mut new_running_web_apps: RunningWebApps = HashMap::new();

    for (holochain_version, summary) in summaries {
      let running_apps_with_port: HashMap<String, WebAppConfig> = HashMap::new();

      for (app_id, running_app) in summary.running_apps {
        if let ManagedApp::WebApp { path_to_web_app } = running_app {
          let port = existing_port(&self.running_apps, &holochain_version, &app_id)
            .unwrap_or_else(|| portpicker::pick_unused_port().expect("No ports free"));

          running_apps_with_port.insert(
            app_id,
            WebAppConfig {
              app_ui_port: port,
              path_to_web_app,
            },
          );
        }
      }
    }

    let new_caddyfile = build_caddyfile_contents(self.caddy_admin_port, new_running_web_apps);

    let caddyfile_path = caddyfile_path(self.environment_path.clone());

    fs::write(caddyfile_path, new_caddyfile).expect("Could not write Caddyfile");

    reload_caddy(caddyfile_path)?;

    Ok(())
  }
}

fn caddyfile_path(environment_path: PathBuf) -> PathBuf {
  environment_path.join("Caddyfile")
}

fn existing_port(
  running_web_apps: &RunningWebApps,
  holochain_version: &HolochainVersion,
  app_id: &String,
) -> Option<u16> {
  if let Some(apps) = running_web_apps.get(&holochain_version) {
    if let Some(app_config) = apps.get(app_id) {
      return Some(app_config.app_ui_port);
    }
  }
  None
}
