use std::process::Command;
use serde::{Deserialize, Serialize};
use dioxus::logger::tracing::error;

use super::Setting;

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
    pub fn from_command(command: &str) -> Result<Plugin, &str> {
        let mut command = Command::new(command);
        let stdout = match command.arg("--json").output() {
            Ok(v) => v.stdout,
            Err(_) => {
                error!("Can't get plugin output data");
                return Err("Can't get plugin output data")
            }
        };
        let json = match str::from_utf8(&stdout) {
            Ok(v) => v,
            Err(_) => {
                error!("Can't read plugin output data");
                return Err("Can't read plugin output data")
            },
        };
        let plugin: Plugin = match serde_json::from_str(json) {
            Ok(v) => v,
            Err(e) => {
                error!("Parsing plugin data error, {}", e);
                return Err("Parsing plugin data error")
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
}
