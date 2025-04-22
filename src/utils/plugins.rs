use dioxus::logger::tracing::{error, info};

use crate::models::Plugin;

/// Load all nmp-plugins installed by user
pub fn load_plugins() -> Vec<Plugin> {
    // TODO: Load plugins from user system.
    info!("Loading plugins...");
    let plugin = Plugin::from_command("../nmp-settings-plugin-example/target/debug/nmp-settings-plugin-example");

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