use std::collections::HashMap;

use dioxus::hooks::{use_context, use_context_provider};
use dioxus::logger::tracing::info;
use dioxus::signals::Signal;
use crate::models::{Plugin, Setting, SettingsCategory};

use super::PluginsStore;

/// Settings of plugins
#[derive(Clone)]
pub struct SettingsStore {
    /// Settings by user friendly category
    categorized_settings: Signal<HashMap<SettingsCategory, Vec<StoredSetting>>>,
    /// Settings by owner
    uncategorized_settings: HashMap<String, Vec<StoredSetting>>,
}

impl SettingsStore {
    pub fn new(categorized_settings: HashMap<SettingsCategory, Vec<StoredSetting>>, uncategorized_settings: HashMap<String, Vec<StoredSetting>>) -> SettingsStore {
        SettingsStore { categorized_settings: Signal::new(categorized_settings), uncategorized_settings }
    }
    pub fn categorized_settings(&self) -> Signal<HashMap<SettingsCategory, Vec<StoredSetting>>> {
        self.categorized_settings
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct StoredSetting {
    owner: String,
    setting: Setting,
}

impl StoredSetting {
    pub fn new(owner: String, setting: Setting) -> StoredSetting {
        StoredSetting { owner, setting }
    }
    pub fn owner(&self) -> &String {
        &self.owner
    }
    pub fn setting(&self) -> &Setting {
        &self.setting
    }
}

/// Plugins Settings store. Lives only until the component is dropped.
pub fn use_settings_store() {
    info!("Init settings store...");
    let plugins = use_context::<PluginsStore>().get_plugins().clone();
    let result = get_category_settings(plugins);
    use_context_provider(|| SettingsStore::new(result.0, result.1));
}

fn get_category_settings(plugins: Vec<Plugin>) -> (HashMap<SettingsCategory, Vec<StoredSetting>>, HashMap<String, Vec<StoredSetting>>) {
    let mut categorized: HashMap<SettingsCategory, Vec<StoredSetting>> = HashMap::new();
    let mut uncategorized: HashMap<String, Vec<StoredSetting>> = HashMap::new();
    
    plugins.iter().for_each(|plugin| {
        let plugin_name = plugin.get_name();
        plugin.get_settings().iter().for_each(|setting| {
            match setting.category() {
                Some(category) => {
                    match categorized.get_mut(&category) {
                        Some(category_settings) => category_settings.push(
                            StoredSetting::new(plugin_name.clone(), setting.clone())
                        ),
                        None => {
                            categorized.insert(
                                category.clone(),
                                vec![StoredSetting::new(plugin_name.clone(), setting.clone())],
                            );
                        },
                    }
                },
                None => {
                    match uncategorized.get_mut(&plugin_name) {
                        Some(plugin_settings) => plugin_settings.push(
                            StoredSetting::new(plugin_name.clone(), setting.clone())
                        ),
                        None => {
                            uncategorized.insert(
                                plugin_name.clone(),
                                vec![StoredSetting::new(plugin_name.clone(), setting.clone())]
                            );
                        }
                    }
                }
            }
        });
    });

    (categorized, uncategorized)
}
