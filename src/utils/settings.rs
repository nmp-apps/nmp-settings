use std::collections::HashMap;

use dioxus::{logger::tracing::error, signals::{Signal, Writable}};

use crate::{models::{Plugin, SettingsCategory}, stores::{SettingsStore, StoredSetting}};

/// Parses loaded plugins data and sorts settings into categories -> (categorized, uncategorized)
pub fn parse_settings_from_plugins(plugins: Vec<Plugin>) -> (HashMap<SettingsCategory, Vec<StoredSetting>>, HashMap<String, Vec<StoredSetting>>) {
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

pub fn mutate_setting_value<F, R>(
    setting: &StoredSetting,
    settings_store: &mut SettingsStore,
    callback: F
) -> ()
where
    F: FnOnce(&mut StoredSetting) -> R
{
    match setting.setting().category().clone() {
        // categorized
        Some(category) => {
            let mut settings = settings_store
                .categorized_settings_mut()
                .write();
            match settings.get_mut(&category) {
                Some(list) => {
                    match list.iter_mut().find(|stored_setting| {
                        stored_setting.id() == setting.id()
                    }) {
                        Some(target_setting) => {
                            callback(target_setting);
                        },
                        None => {
                            error!("Can't find list of settings by id \"{}\"", setting.id());
                        }
                    }
                },
                None => {
                    error!("Can't find list of settings by category \"{}\"", category.get_name());
                }
            }
        },
        // uncategorized
        None => {
            let mut settings = settings_store
                .uncategorized_settings_mut()
                .write();
            match settings.get_mut(setting.owner()) {
                Some(list) => {
                    match list.iter_mut().find(|stored_setting| {
                        stored_setting.id() == setting.id()
                    }) {
                        Some(target_setting) => {
                            callback(target_setting);
                        },
                        None => {
                            error!("Can't find list of settings by id \"{}\"", setting.id());
                        }
                    }
                },
                None => {
                    error!("Can't find list of settings by owner category \"{}\"", setting.owner());
                }
            }

        },
    }
}