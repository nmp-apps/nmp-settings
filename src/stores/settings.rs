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
    let result = get_category_settings(plugins);
    use_context_provider(|| SettingsStore::new(result.0, result.1));
}

/// Parses loaded plugins data and sorts settings into categories
fn get_category_settings(plugins: Vec<Plugin>) -> (HashMap<SettingsCategory, Vec<StoredSetting>>, HashMap<String, Vec<StoredSetting>>) {
    let mut categorized: HashMap<SettingsCategory, Vec<StoredSetting>> = HashMap::new();
    let mut uncategorized: HashMap<String, Vec<StoredSetting>> = HashMap::new();
    
    plugins.iter().for_each(|plugin| {
        let plugin_name = plugin.get_name();
        plugin.get_settings().iter().for_each(|setting| {
            match setting.category() {
                // categorized setting
                Some(category) => {
                    match categorized.get_mut(&category) {
                        // list exists
                        Some(category_settings) => {
                            let setting_id = format!("{}:{}", category.get_id(), category_settings.len() + 1);
                            category_settings.push(
                                StoredSetting::new(
                                    setting_id,
                                    plugin_name.clone(), 
                                    setting.clone())
                            )
                        },
                        // new list
                        None => {
                            let setting_id = format!("{}:{}", category.get_id(), 1);
                            categorized.insert(
                                category.clone(),
                                vec![
                                    StoredSetting::new(
                                        setting_id,
                                        plugin_name.clone(),
                                        setting.clone()
                                    )
                                ],
                            );
                        },
                    }
                },
                // setting in plugin category
                None => {
                    match uncategorized.get_mut(&plugin_name) {
                        // list exists
                        Some(plugin_settings) => {
                            let setting_id = format!("{}:{}", plugin_name.clone(), plugin_settings.len() + 1);
                            plugin_settings.push(
                                StoredSetting::new(
                                    setting_id,
                                    plugin_name.clone(), 
                                    setting.clone()
                                )
                            )
                        },
                        // new list
                        None => {
                            let setting_id = format!("{}:{}", plugin_name.clone(), 1);
                            uncategorized.insert(
                                plugin_name.clone(),
                                vec![
                                    StoredSetting::new(
                                        setting_id,
                                        plugin_name.clone(),
                                        setting.clone()
                                    )
                                ]
                            );
                        }
                    }
                }
            }
        });
    });

    (categorized, uncategorized)
}
