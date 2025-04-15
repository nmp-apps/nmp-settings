use dioxus::hooks::use_context_provider;
use dioxus::logger::tracing::{error, info, trace};

use crate::models::Plugin;

/// Initial Plugins Data. Immutable!
#[derive(Clone)]
pub struct PluginsStore {
    plugins: Vec<Plugin>
}

impl PluginsStore {
    pub fn new(plugins: Vec<Plugin>) -> PluginsStore {
        PluginsStore { plugins }
    }
    pub fn get_plugins(&self) -> &Vec<Plugin> {
        &self.plugins
    }
}

/// Plugins store. Immutable! Lives only until the component is dropped.
pub fn use_plugins_store() {
    trace!("Init settings store...");
    use_context_provider(|| PluginsStore::new(load_plugins()));
}

/// Load all nmp-plugins installed by user
pub fn load_plugins() -> Vec<Plugin> {
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