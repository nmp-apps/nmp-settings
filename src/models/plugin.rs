use std::process::Command;
use serde::{Deserialize, Serialize};
use dioxus::logger::tracing::{error, warn};

use crate::utils::{plugin_check_version, truncate_plugin_name};

use super::{DisabledPlugin, Setting};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
    plugin_version: String,
    nmp_settings_version: String,
    plugin_name: String,
    settings: Vec<Setting>,
}

impl Plugin {
    // pub fn new(plugin_version: String, nmp_settings_version: String, plugin_name: String, settings: Vec<Settings>,) -> Plugin {
    //     Plugin {
    //         plugin_version,
    //         nmp_settings_version,
    //         plugin_name,
    //         settings,
    //     }
    // }
    pub fn from_command(command_name: &str) -> Result<Plugin, DisabledPlugin> {
        
        let is_check_version_valid = plugin_check_version(command_name);
        
        if !is_check_version_valid {
            warn!("Plugin version check by command {} is not valid.", command_name);
            return Err(DisabledPlugin::new(truncate_plugin_name(command_name), None, None));
        }

        let mut command = Command::new(command_name);

        let stdout = match command.arg("--json").output() {
            Ok(v) => v.stdout,
            Err(e) => {
                error!("Can't get plugin {} output data: {}", command_name, e);
                return Err(DisabledPlugin::new(truncate_plugin_name(command_name), None, None));
            }
        };
        let json = match str::from_utf8(&stdout) {
            Ok(v) => v,
            Err(e) => {
                error!("Can't read plugin {} output data {}", command_name, e);
                return Err(DisabledPlugin::new(truncate_plugin_name(command_name), None, None));
            },
        };
        let plugin: Plugin = match serde_json::from_str(json) {
            Ok(v) => v,
            Err(e) => {
                error!("Parsing plugin {} data error: {}", command_name, e);
                return Err(DisabledPlugin::new(truncate_plugin_name(command_name), None, None));
            },
        };
        Ok(plugin)
    }
    pub fn from_str(plugin_name: &str, json: &str) -> Result<Plugin, DisabledPlugin> {
        let plugin: Plugin = match serde_json::from_str(json) {
            Ok(v) => v,
            Err(e) => {
                error!("Parsing plugin data error, {}", e);
                return Err(DisabledPlugin::new(truncate_plugin_name(plugin_name), None, None));
            },
        };
        Ok(plugin)
    }
    fn check_version(&self) -> bool {
        todo!("Check version is not implemented yet");
    }
    pub fn get_name(&self) -> String {
        self.plugin_name.replace("nmp-settings-plugin-", "")
    }
    pub fn get_settings(&self) -> &Vec<Setting> {
        &self.settings
    }
    pub fn get_settings_mut(&mut self) -> &mut Vec<Setting> {
        &mut self.settings
    }
}
