use dioxus::hooks::use_context_provider;

use crate::models::Plugin;
use crate::utils::load_plugins;

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

pub fn use_plugins_store() {
    use_context_provider(|| PluginsStore::new(load_plugins()));
}