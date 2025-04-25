use std::{collections::HashMap, process::Command};

use dioxus::logger::tracing::{error, info};

use crate::models::Plugin;

const TEST_PLUGIN_PATH: &str = "../nmp-settings-plugin-example/target/debug/nmp-settings-plugin-example";

/// Load all nmp-plugins installed by user
pub fn load_plugins() -> Vec<Plugin> {
    // TODO: Load plugins from user system.
    // Split dev (examples plugin only) and prod functions.
    info!("Loading plugins...");
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

// TODO: leave this version for dev and make a version for prod build
pub fn send_data_to_plugins(changed_plugins: HashMap<String, Plugin>) {
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
        let _ = command
            .arg("--json")
            .arg(data)
            .spawn();
    }
}