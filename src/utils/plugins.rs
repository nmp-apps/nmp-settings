use std::{collections::HashMap, process::Command};

use dioxus::logger::tracing::{error, info};

use crate::models::Plugin;

#[cfg(not(debug_assertions))]
use super::find_apps_by_name;

#[cfg(debug_assertions)]
const TEST_PLUGIN_PATH: &str = "../nmp-settings-plugin-example/target/debug/nmp-settings-plugin-example";

#[cfg(debug_assertions)]
pub fn load_plugins(plugins: Option<Vec<String>>) -> Vec<Plugin> {
    info!("Loading plugins...");
    match plugins {
        Some(plugin_names) => {
            let mut plugin_list = vec![];
            plugin_names.iter().for_each(|_| {
                let plugin = Plugin::from_command(TEST_PLUGIN_PATH);

                match plugin {
                    Ok(plugin) => {
                        plugin_list.push(plugin);
                    },
                    Err(e) => {
                        error!(e);
                    },
                }
            });
            plugin_list
        },
        // find plugins in system and load them [TODO]
        None => {
            let plugin = Plugin::from_command(TEST_PLUGIN_PATH);

            match plugin {
                Ok(plugin) => {
                    vec![plugin]
                },
                Err(e) => {
                    error!(e);
                    vec![]
                },
            }
        }
    }
    
}

/// In release version takes plugins installed by user
#[cfg(not(debug_assertions))]
pub fn load_plugins(plugins: Option<Vec<String>>) -> Vec<Plugin> {
    info!("Loading plugins...");
    match plugins {
        Some(plugin_names) => {
            let full_names = plugin_names.iter().map(|name| {
                format!("nmp-settings-plugin-{}", name)
            });
            let mut plugin_list = vec![];
            full_names.for_each(|name| {
                let plugin = Plugin::from_command(
                    name.as_str()
                );

                match plugin {
                    Ok(plugin) => {
                        plugin_list.push(plugin);
                    },
                    Err(e) => {
                        error!(e);
                    },
                }
            });
            plugin_list
        },
        None => {
            let plugin_paths: Vec<String> = find_apps_by_name("nmp-settings-plugin-");
            let mut plugins: Vec<Plugin> = vec![];

            plugin_paths.iter().for_each(|plugin_path| {
                let plugin = Plugin::from_command(plugin_path);
                match plugin {
                    Ok(plugin) => {
                        plugins.push(plugin);
                    },
                    Err(e) => {
                        error!(e);
                    },
                }

            });
            plugins
        }
    }    
}

/// Receives JSONs and returns Plugins
pub fn parse_plugins(plugins_data: Vec<String>) -> Vec<Plugin> {
    let mut plugins: Vec<Plugin> = vec![];
    for plugin_data in plugins_data {
        let plugin_result = Plugin::from_str(&plugin_data);
        match plugin_result {
            Ok(plugin) => plugins.push(plugin),
            Err(_) => ()
        }
    }
    plugins
}

#[cfg(debug_assertions)]
pub fn send_data_to_plugins(changed_plugins: &HashMap<String, Plugin>) -> Vec<String> {
    let mut updated_plugins_data = vec![];
    for (plugin_name, plugin) in changed_plugins {
        let serialized_plugin = serde_json::to_string(&plugin);
        let data = match serialized_plugin {
            Ok(data) => data,
            Err(err) => {
                error!("{:?}", err);
                continue;
            }
        };
        let plugin_name = TEST_PLUGIN_PATH;
        let mut command = Command::new(plugin_name);
        let response = command
            .arg("--json")
            .arg(data)
            .output();
        let stdout = match response {
            Ok(output) => output.stdout,
            Err(err) => {
                error!("Failed on updating plugins data:\n{:#?}", err);
                continue;
            }
        };
        let json_data = match str::from_utf8(&stdout) {
            Ok(v) => v,
            Err(_) => {
                error!("Can't read plugin output data");
                continue;
            },
        };
        updated_plugins_data.push(String::from(json_data));
    }
    updated_plugins_data
}

/// In release version takes plugins installed by user
#[cfg(not(debug_assertions))]
pub fn send_data_to_plugins(changed_plugins: &HashMap<String, Plugin>) -> Vec<String> {
    let mut updated_plugins_data = vec![];
    for (plugin_name, plugin) in changed_plugins {
        let serialized_plugin = serde_json::to_string(&plugin);
        let data = match serialized_plugin {
            Ok(data) => data,
            Err(err) => {
                error!("{:?}", err);
                continue;
            }
        };
        let plugin_name = format!("nmp-settings-plugin-{}", plugin_name);
        let mut command = Command::new(plugin_name);
        let response = command
            .arg("--json")
            .arg(data)
            .output();
        let stdout = match response {
            Ok(output) => output.stdout,
            Err(err) => {
                error!("Failed on updating plugins data:\n{:#?}", err);
                continue;
            }
        };
        let json_data = match str::from_utf8(&stdout) {
            Ok(v) => v,
            Err(_) => {
                error!("Can't read plugin output data");
                continue;
            },
        };
        updated_plugins_data.push(String::from(json_data));
    }
    updated_plugins_data
}