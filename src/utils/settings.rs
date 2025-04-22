use std::collections::HashMap;

use crate::models::{Plugin, SettingsCategory};
use crate::stores::StoredSetting;

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