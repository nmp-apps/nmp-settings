use std::collections::HashMap;
use std::ops::Deref;

use dioxus::hooks::{use_context, use_context_provider};
use dioxus::logger::tracing::info;
use dioxus::signals::Signal;
use crate::models::{Plugin, Setting, SettingsCategory};
use crate::utils::parse_settings_from_plugins;

use super::PluginsStore;

/// Settings of plugins
#[derive(Clone)]
pub struct SettingsStore {
    /// Settings by user friendly category
    categorized_settings: Signal<HashMap<SettingsCategory, Vec<StoredSetting>>>,
    /// Settings by owner
    uncategorized_settings: Signal<HashMap<String, Vec<StoredSetting>>>,
}

impl SettingsStore {
    pub fn new(categorized_settings: HashMap<SettingsCategory, Vec<StoredSetting>>, uncategorized_settings: HashMap<String, Vec<StoredSetting>>) -> SettingsStore {
        SettingsStore { categorized_settings: Signal::new(categorized_settings), uncategorized_settings: Signal::new(uncategorized_settings) }
    }
    pub fn categorized_settings(&self) -> &Signal<HashMap<SettingsCategory, Vec<StoredSetting>>> {
        &self.categorized_settings
    }
    pub fn categorized_settings_mut(&mut self) -> &mut Signal<HashMap<SettingsCategory, Vec<StoredSetting>>> {
        &mut self.categorized_settings
    }
    pub fn uncategorized_settings(&self) -> Signal<HashMap<String, Vec<StoredSetting>>> {
        self.uncategorized_settings
    }
    pub fn uncategorized_settings_mut(&mut self) -> &mut Signal<HashMap<String, Vec<StoredSetting>>> {
        &mut self.uncategorized_settings
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StoredSetting {
    id: String,
    owner: String,
    setting: Setting,
}

impl StoredSetting {
    pub fn new(id: String, owner: String, setting: Setting) -> StoredSetting {
        StoredSetting { id, owner, setting }
    }
    pub fn id(&self) -> &String {
        &self.id
    }
    pub fn owner(&self) -> &String {
        &self.owner
    }
    pub fn setting(&self) -> &Setting {
        &self.setting
    }
    pub fn setting_mut(&mut self) -> &mut Setting {
        &mut self.setting
    }
}

/// Plugins Settings store. Lives only until the component is dropped.
pub fn use_settings_store() {
    info!("Init settings store...");
    let plugins = use_context::<PluginsStore>().get_plugins().clone();
    let parsed_settings = parse_settings_from_plugins(plugins);
    use_context_provider(|| SettingsStore::new(parsed_settings.0, parsed_settings.1));
}
