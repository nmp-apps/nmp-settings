use std::collections::HashMap;

use dioxus::logger::tracing::error;
use dioxus::prelude::*;

use crate::models::{Plugin, Setting};
use crate::stores::{PluginsStore, SettingsStore};
use crate::components::Button;
use crate::utils::{load_plugins, parse_plugins, parse_settings_from_plugins, send_data_to_plugins};

#[component]
pub fn SaveSettingsButton() -> Element {
    let mut settings_store = use_context::<SettingsStore>();
    let mut plugins_store = use_context::<PluginsStore>();

    let settings_store_clone = settings_store.clone();
    let is_disabled = use_memo(move || {
        settings_store_clone.changed_settings().read().len() <= 0
    });

    let handler = move |_| {
        let mut changed_plugins: HashMap<String, Plugin> = HashMap::new();
        for (setting_id, setting) in settings_store.changed_settings().read().iter() {
            let parsed_id: Vec<&str> = setting_id.split(":").collect();
            let plugin_name = match parsed_id.get(0) {
                Some(v) => v,
                None => {
                    error!("Plugin name not found when parsing setting id");
                    continue;
                },
            };
            
            let plugins = plugins_store.get_plugins_mut();
            let plugin = match plugins.iter_mut().find(|plugin| plugin.get_name() == String::from(*plugin_name)) {
                Some(plugin) => plugin,
                None => {
                    error!("Plugin [{}] not found", plugin_name);
                    continue;
                }
            };
            
            match changed_plugins.get_mut(*plugin_name) {
                Some(plugin) => {
                    // find setting in plugin and change it
                    let target_setting: Option<&mut Setting> = find_setting(plugin, setting);
                    let stored_setting = settings_store.find_setting(setting, &String::from(*plugin_name));
                    let target_setting = match target_setting {
                        Some(v) => { v },
                        None => {
                            error!("Setting not found in plugin!");
                            continue;
                        }
                    };
                    let stored_setting = match stored_setting {
                        Some(v) => v,
                        None => {
                            error!("Setting not found in stored settings!");
                            continue;
                        }
                    };
                    let target_component = target_setting.component_mut();
                    *target_component = stored_setting.setting().component().clone();
                },
                None => {
                    // change plugin setting and save it in changed_plugins
                    let target_setting: Option<&mut Setting> = find_setting(plugin, setting);
                    let stored_setting = settings_store.find_setting(setting, &String::from(*plugin_name));
                    let target_setting = match target_setting {
                        Some(v) => { v },
                        None => {
                            error!("Setting not found in plugin!");
                            continue;
                        }
                    };
                    let stored_setting = match stored_setting {
                        Some(v) => v,
                        None => {
                            error!("Setting not found in stored settings!");
                            continue;
                        }
                    };
                    let target_component = target_setting.component_mut();
                    *target_component = stored_setting.setting().component().clone();
                    changed_plugins.insert(String::from(*plugin_name), plugin.clone());
                }
            }
        }

        let mut updated_plugins_data: Vec<String> = vec![];
        if changed_plugins.len() > 0 {
            updated_plugins_data = send_data_to_plugins(&changed_plugins);
        }
        let settings_store_mut = &mut settings_store;
        
        {
            let mut changed_settings = settings_store_mut.changed_settings_mut().write();
            changed_settings.clear();
        }
        
        let response_plugins = parse_plugins(updated_plugins_data);
        response_plugins.iter().for_each(|plugin| {
            plugins_store.set_plugin(plugin.clone());
        });
        // parse all settings to save settings order 
        let stored_plugins = plugins_store.get_plugins().clone();
        let (parsed_categorized, parsed_uncategorized) = parse_settings_from_plugins(stored_plugins);

        {
            let categorized_settings_mut = settings_store_mut.categorized_settings_mut();
            categorized_settings_mut.set(parsed_categorized);
        }
        {
            let uncategorized_settings_mut = settings_store_mut.uncategorized_settings_mut();
            uncategorized_settings_mut.set(parsed_uncategorized);
        }
    };

     rsx! {
        Button {
            title: "Save changes",
            disabled: is_disabled(),
            onclick: handler,
            primary: true,
            "Save"
        }
    }
}

pub fn find_setting<'a>(plugin: &'a mut Plugin, setting: &Setting) -> Option<&'a mut Setting> {
    plugin.get_settings_mut().iter_mut().find(|plugin_setting| {
        plugin_setting.category() == setting.category() &&
        plugin_setting.title() == setting.title()
    })

}