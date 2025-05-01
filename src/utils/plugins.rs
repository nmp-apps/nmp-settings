use std::{collections::HashMap, process::Command, env};

use dioxus::logger::tracing::{error, info};

use crate::models::{Plugin, DisabledPlugin};

#[cfg(not(debug_assertions))]
use super::find_apps_by_name;

#[cfg(debug_assertions)]
pub fn load_plugins(plugins: Option<Vec<String>>) -> (Vec<Plugin>, Vec<DisabledPlugin>) {
    let test_plugin_path: String = env::var("TEST_PLUGIN_PATH").unwrap_or(String::new());
    
    info!("Loading plugins...");
    let mut enabled_plugins: Vec<Plugin> = vec![];
    let mut disabled_plugins: Vec<DisabledPlugin> = vec![];

    match plugins {
        Some(plugin_names) => {
            plugin_names.iter().for_each(|_| {
                let plugin = Plugin::from_command(test_plugin_path.as_str());

                match plugin {
                    Ok(plugin) => {
                        enabled_plugins.push(plugin);
                    },
                    Err(disabled_plugin) => {
                        error!("Found {:?}", disabled_plugin);
                        disabled_plugins.push(disabled_plugin);
                    },
                }
            });
        },
        // find all plugins in system and load them
        None => {
            let plugin = Plugin::from_command(test_plugin_path.as_str());
            

            match plugin {
                Ok(plugin) => {
                    enabled_plugins.push(plugin);
                },
                Err(disabled_plugin) => {
                    error!("Found {:?}", disabled_plugin);
                    disabled_plugins.push(disabled_plugin);
                },
            }
        }
    }
    (enabled_plugins, disabled_plugins)
    
}

/// In release version takes plugins installed by user
#[cfg(not(debug_assertions))]
pub fn load_plugins(plugins: Option<Vec<String>>) -> (Vec<Plugin>, Vec<DisabledPlugin>) {
    info!("Loading plugins...");
        let mut enabled_plugins: Vec<Plugin> = vec![];
        let mut disabled_plugins: Vec<DisabledPlugin> = vec![];
    match plugins {
        Some(plugin_names) => {
            let full_names = plugin_names.iter().map(|name| {
                format!("nmp-settings-plugin-{}", name)
            });
            full_names.for_each(|name| {
                let plugin = Plugin::from_command(
                    name.as_str()
                );

                match plugin {
                    Ok(plugin) => {
                        enabled_plugins.push(plugin);
                    },
                    Err(disabled_plugin) => {
                        error!("Found {:?}", disabled_plugin);
                        disabled_plugins.push(disabled_plugin);
                    },
                }
            });
        },
        None => {
            let plugin_paths: Vec<String> = find_apps_by_name("nmp-settings-plugin-");

            plugin_paths.iter().for_each(|plugin_path| {
                let plugin = Plugin::from_command(plugin_path);
                match plugin {
                    Ok(plugin) => {
                        enabled_plugins.push(plugin);
                    },
                    Err(disabled_plugin) => {
                        error!("Found {:?}", disabled_plugin);
                        disabled_plugins.push(disabled_plugin);
                    },
                }

            });
        }
    }
    (enabled_plugins, disabled_plugins)

}

/// Receives JSONs and returns Plugins
pub fn parse_plugins(plugins_data: HashMap<String, String>) -> (Vec<Plugin>, Vec<DisabledPlugin>) {
    let mut plugins: Vec<Plugin> = vec![];
    let mut disabled_plugins: Vec<DisabledPlugin> = vec![];
    for (plugin_name, plugin_data) in plugins_data {
        let plugin_result = Plugin::from_str(&plugin_name, &plugin_data);
        match plugin_result {
            Ok(plugin) => plugins.push(plugin),
            Err(disabled_plugin) => disabled_plugins.push(disabled_plugin), 
        }
    }
    (plugins, disabled_plugins)
}

// TODO: add disabling plugins on error
/// Send data to plugins, return updated plugin JSON data
#[cfg(debug_assertions)]
pub fn send_data_to_plugins(changed_plugins: &HashMap<String, Plugin>) -> (HashMap<String, String>, Vec<DisabledPlugin>) {
    let test_plugin_path: String = env::var("TEST_PLUGIN_PATH").unwrap_or(String::new());
    
    let mut updated_plugins_data: HashMap<String, String> = HashMap::new();
    let mut disabled_plugins = vec![];
    for (plugin_name, plugin) in changed_plugins {
        let serialized_plugin = serde_json::to_string(&plugin);
        let data = match serialized_plugin {
            Ok(data) => data,
            Err(err) => {
                error!("{:?}", err);
                disabled_plugins.push(DisabledPlugin::new(truncate_plugin_name(&plugin_name), None, None));
                continue;
            }
        };
        let mut command = Command::new(test_plugin_path.as_str());
        let response = command
            .arg("--json")
            .arg(data)
            .output();
        let stdout = match response {
            Ok(output) => output.stdout,
            Err(err) => {
                error!("Failed on updating plugins data:\n{:#?}", err);
                disabled_plugins.push(DisabledPlugin::new(truncate_plugin_name(&plugin_name), None, None));
                continue;
            }
        };
        let json_data = match str::from_utf8(&stdout) {
            Ok(v) => v,
            Err(_) => {
                error!("Can't read plugin output data");
                disabled_plugins.push(DisabledPlugin::new(truncate_plugin_name(&plugin_name), None, None));
                continue;
            },
        };
        updated_plugins_data.insert(plugin_name.clone(), String::from(json_data));
    }
    (updated_plugins_data, disabled_plugins)
}

/// In release version takes plugins installed by user
#[cfg(not(debug_assertions))]
pub fn send_data_to_plugins(changed_plugins: &HashMap<String, Plugin>) -> (HashMap<String, String>, Vec<DisabledPlugin>) {
    let mut updated_plugins_data: HashMap<String, String> = HashMap::new();
    let mut disabled_plugins = vec![];
    for (plugin_name, plugin) in changed_plugins {
        let serialized_plugin = serde_json::to_string(&plugin);
        let data = match serialized_plugin {
            Ok(data) => data,
            Err(err) => {
                error!("{:?}", err);
                disabled_plugins.push(DisabledPlugin::new(truncate_plugin_name(&plugin_name), None, None));
                continue;
            }
        };
        let command_name = format!("nmp-settings-plugin-{}", plugin_name);
        let mut command = Command::new(command_name);
        let response = command
            .arg("--json")
            .arg(data)
            .output();
        let stdout = match response {
            Ok(output) => output.stdout,
            Err(err) => {
                error!("Failed on updating plugins data:\n{:#?}", err);
                disabled_plugins.push(DisabledPlugin::new(truncate_plugin_name(&plugin_name), None, None));
                continue;
            }
        };
        let json_data = match str::from_utf8(&stdout) {
            Ok(v) => v,
            Err(_) => {
                error!("Can't read plugin output data");
                disabled_plugins.push(DisabledPlugin::new(truncate_plugin_name(&plugin_name), None, None));
                continue;
            },
        };
        updated_plugins_data.insert(plugin_name.clone(),String::from(json_data));
    }
    (updated_plugins_data, disabled_plugins)
}

pub fn truncate_plugin_name(full_name: &str) -> String {
    full_name.replace("nmp-settings-plugin-", "")
}

// Tries to run user plugin with `--check-version` flag and handle it's output.
pub fn plugin_check_version(command_name: &str) -> bool {
    let mut plugin_command = Command::new(command_name);
    let check_version_utf8 = match plugin_command.arg("--check-version").output() {
        Ok(output) => output.stdout,
        Err(err) => {
            error!("Plugin by command {} check version failed: {:?}", command_name, err);
            return false;
        }
    };
    let check_version_json = match str::from_utf8(&check_version_utf8) {
        Ok(json) => json,
        Err(err) => {
            error!("Can't read plugin version output by command {}: {:?}", command_name, err);
            return false;
        }
    };
    let is_check_version_valid: bool = match serde_json::from_str(check_version_json) {
        Ok(v) => v,
        Err(err) => {
            error!("Parsing JSON plugin version by command {} failed: {:?}", command_name, err);
            return false;
        }
    };

    is_check_version_valid
}