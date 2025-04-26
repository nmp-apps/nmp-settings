use dioxus::hooks::use_context_provider;
use dioxus::logger::tracing::{error, trace};

use crate::models::Plugin;
use crate::utils::load_plugins;

/// Initial Plugins Data
#[derive(Clone, Debug)]
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
    pub fn get_plugins_mut(&mut self) -> &mut Vec<Plugin> {
        &mut self.plugins
    }
    pub fn set_plugin(&mut self, plugin: Plugin) {
        let target_plugin = self
            .get_plugins_mut()
            .iter_mut()
            .find(|stored_plugin| stored_plugin.get_name() == plugin.get_name());
        match target_plugin {
            Some(stored_plugin) => {
                *stored_plugin = plugin;
            },
            None => {
                error!("Plugin [{}] not found.", plugin.get_name());
            }
        }
    }
}

/// Plugins store. Immutable! Lives only until the component is dropped.
pub fn use_plugins_store() {
    trace!("Init settings store...");
    use_context_provider(|| PluginsStore::new(load_plugins(None)));
}
