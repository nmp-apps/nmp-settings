use std::process::Command;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use dioxus::prelude::*;
use dioxus::{logger::tracing::{error, warn}, signals::Signal};

use crate::models::Notification;
use crate::{stores::AppStore, utils::{plugin_check_version, truncate_plugin_name}};

use super::{DisabledPlugin, Setting};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
    plugin_version: String,
    nmp_settings_version: String,
    plugin_name: String,
    settings: Vec<Setting>,
}

impl Plugin {
    pub fn from_command(command_name: &str) -> Result<Plugin, DisabledPlugin> {
        let mut app_store = use_context::<Signal<AppStore>>();
        let is_check_version_valid = plugin_check_version(command_name);
        
        if !is_check_version_valid {
            warn!("Plugin version check by command {} is not valid.", command_name);
            app_store.write().push_notification(
                &Notification::new(
                    String::from("Plugin version check failed."),
                    Some(format!("Plugin {} failed version check.", command_name)),
                    None,
                    Some(5000),
                ),
            );
            return Err(DisabledPlugin::new(truncate_plugin_name(command_name), None, None));
        }

        let mut command = Command::new(command_name);

        let stdout = match command.arg("--json").output() {
            Ok(v) => v.stdout,
            Err(e) => {
                error!("Can't get plugin {} output data: {}", command_name, e);
                app_store.write().push_notification(
                    &Notification::new(
                        String::from("Loading plugin data error"),
                        Some(format!("Plugin {} has incorrect data.", command_name)),
                        None,
                        Some(5000),
                    ),
                );
                return Err(DisabledPlugin::new(truncate_plugin_name(command_name), None, None));
            }
        };
        let json = match str::from_utf8(&stdout) {
            Ok(v) => v,
            Err(e) => {
                error!("Can't read plugin {} output data {}", command_name, e);
                app_store.write().push_notification(
                    &Notification::new(
                        String::from("Loading plugin data error"),
                        Some(format!("Plugin {} has incorrect data.", command_name)),
                        None,
                        Some(5000),
                    ),
                );
                return Err(DisabledPlugin::new(truncate_plugin_name(command_name), None, None));
            },
        };
        let plugin: Plugin = match serde_json::from_str(json) {
            Ok(v) => v,
            Err(e) => {
                error!("Parsing plugin {} data error: {}", command_name, e);
                app_store.write().push_notification(
                    &Notification::new(
                        String::from("Loading plugin data error"),
                        Some(format!("Plugin {} has incorrect data.", command_name)),
                        None,
                        Some(5000),
                    ),
                );
                return Err(DisabledPlugin::new(truncate_plugin_name(command_name), None, None));
            },
        };
        Ok(plugin)
    }
    pub fn from_str(plugin_name: &str, json: &str) -> Result<Plugin, DisabledPlugin> {
        let mut app_store = use_context::<Signal<AppStore>>();
        let plugin: Plugin = match serde_json::from_str(json) {
            Ok(v) => v,
            Err(e) => {
                error!("Parsing plugin data error, {}", e);
                app_store.write().push_notification(
                    &Notification::new(
                        String::from("Loading plugin data error"),
                        Some(format!("Plugin {} has incorrect data.", plugin_name)),
                        None,
                        Some(5000),
                    ),
                );
                return Err(DisabledPlugin::new(truncate_plugin_name(plugin_name), None, None));
            },
        };
        Ok(plugin)
    }
    /// Checks `nmpSettingsVersion` field of plugin data.
    pub fn check_version_compatibility(&self) -> bool {
        // TODO: now it only checks if version is not higher than users nmp-settings version.
        // Add lower border version check.
        let user_nmp_settings_version = env!("CARGO_PKG_VERSION");
        let installed_version = match Version::parse(user_nmp_settings_version) {
            Ok(version) => version,
            Err(_) => {
                error!("Check version error: parsing version of nmp-settings is failed.");
                return false;
            }
        };
        let required_version = match VersionReq::parse(format!(">={}", self.get_nmp_settings_version()).as_str()) {
            Ok(version) => version,
            Err(_) => {
                error!("Check version error: parsing installed version for plugin {} is failed.", self.get_name());
                return false;
            }
        };
        
        required_version.matches(&installed_version)
    }
    pub fn get_name(&self) -> String {
        self.plugin_name.replace("nmp-settings-plugin-", "")
    }
    pub fn get_version(&self) -> String {
        self.plugin_version.clone()
    }
    pub fn get_nmp_settings_version(&self) -> String {
        self.nmp_settings_version.clone()
    }
    pub fn get_settings(&self) -> &Vec<Setting> {
        &self.settings
    }
    pub fn get_settings_mut(&mut self) -> &mut Vec<Setting> {
        &mut self.settings
    }
}
