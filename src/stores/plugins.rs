use dioxus::hooks::use_context_provider;
use dioxus::logger::tracing::{error, trace};
use dioxus::signals::Signal;

use crate::models::{DisabledPlugin, Plugin};
use crate::utils::load_plugins;

/// Initial Plugins Data
#[derive(PartialEq, Clone, Debug)]
pub struct PluginsStore {
    plugins: Vec<Plugin>,
    disabled_plugins: Signal<Vec<DisabledPlugin>>
}

impl PluginsStore {
    pub fn new(plugins: Vec<Plugin>, disabled_plugins: Vec<DisabledPlugin>) -> PluginsStore {
        PluginsStore { plugins, disabled_plugins: Signal::new(disabled_plugins) }
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
    pub fn get_disabled_plugins(&self) -> Signal<Vec<DisabledPlugin>> {
        self.disabled_plugins
    }
    pub fn get_disabled_plugins_mut(&mut self) -> &mut Signal<Vec<DisabledPlugin>> {
        &mut self.disabled_plugins
    }
}

/// Plugins store. Immutable! Lives only until the component is dropped.
pub fn use_plugins_store() {
    trace!("Init settings store...");
    let plugins = load_plugins(None);
    use_context_provider(|| PluginsStore::new(plugins.0, plugins.1));
}
