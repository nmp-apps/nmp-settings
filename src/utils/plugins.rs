use std::{collections::HashMap, process::Command};

use dioxus::logger::tracing::{error, info};

use crate::models::Plugin;

const TEST_PLUGIN_PATH: &str = "../nmp-settings-plugin-example/target/debug/nmp-settings-plugin-example";

/// Load all nmp-plugins installed by user
pub fn load_plugins(plugins: Option<Vec<String>>) -> Vec<Plugin> {
    // TODO: Load plugins from user system.
    // Split dev (examples plugin only) and prod functions.
    info!("Loading plugins...");
    match plugins {
        Some(plugin_names) => {
            let mut plugin_list = vec![];
            plugin_names.iter().for_each(|name| {
                let plugin = Plugin::from_command(
                    TEST_PLUGIN_PATH // for prod change to "name" argument
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

// TODO: leave this version for dev and make a version for prod build
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