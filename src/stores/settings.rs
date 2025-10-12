use std::collections::HashMap;

use dioxus::hooks::{use_context, use_context_provider};
use dioxus::logger::tracing::{error, info, trace};
use dioxus::signals::{ReadableExt, Signal, WritableExt};
use crate::models::{Setting, SettingComponent, SettingsCategory};
use crate::utils::parse_settings_from_plugins;

use super::PluginsStore;

/// Settings of plugins
#[derive(Clone, Debug)]
pub struct SettingsStore {
    /// Settings by user friendly category
    categorized_settings: Signal<HashMap<SettingsCategory, Vec<StoredSetting>>>,
    /// Settings by owner
    uncategorized_settings: Signal<HashMap<String, Vec<StoredSetting>>>,
    // key is StoredSetting id and value is plugin initial Setting
    changed_settings: Signal<HashMap<String, Setting>>,
    approved_confirmation_setting_ids: Signal<Vec<String>>
}

impl SettingsStore {
    pub fn new(categorized_settings: HashMap<SettingsCategory, Vec<StoredSetting>>, uncategorized_settings: HashMap<String, Vec<StoredSetting>>) -> SettingsStore {
        SettingsStore {
            categorized_settings: Signal::new(categorized_settings),
            uncategorized_settings: Signal::new(uncategorized_settings),
            changed_settings: Signal::new(HashMap::new()),
            approved_confirmation_setting_ids: Signal::new(vec![])
        }
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
    pub fn changed_settings(&self) -> Signal<HashMap<String, Setting>> {
        self.changed_settings
    }
    pub fn changed_settings_mut(&mut self) -> &mut Signal<HashMap<String, Setting>> {
        &mut self.changed_settings
    }
    pub fn approved_confirmation_setting_ids(&self) -> Signal<Vec<String>> {
        self.approved_confirmation_setting_ids
    }
    pub fn approve_confirmation_setting(&mut self, setting_id: &String) {
        if !self.approved_confirmation_setting_ids.read().contains(setting_id) {
            self.approved_confirmation_setting_ids.write().push(setting_id.clone());
        }
    }
    /// Finds setting in categorized or uncategorized lists and returns it's clone
    pub fn find_setting(&self, setting: &Setting, plugin_name: &String) -> Option<StoredSetting>{
        match setting.category() {
            // Try to find in categorized settings
            Some(category) => {
                match self.categorized_settings().read().get(&category) {
                    Some(settings_list) => {
                        settings_list.iter().find(|stored_setting| {
                            stored_setting.setting().category() == setting.category() &&
                            stored_setting.setting().title() == setting.title()
                        }).cloned()
                    },
                    None => {
                        trace!("Setting not found {:#?}", setting);
                        return None;
                    }
                }
            },
            // Try to find by setting owner plugin
            None => {
                match self.uncategorized_settings().read().get(plugin_name) {
                    Some(settings_list) => {
                        settings_list.iter().find(|stored_setting| {
                            stored_setting.setting().category() == setting.category() &&
                            stored_setting.setting().title() == setting.title()
                        }).cloned()
                    },
                    None => {
                        trace!("Setting not found {:#?}", setting);
                        None
                    }
                }
            }
        }
    }
    /// Finds setting in settings store then calls callback with found setting
    pub fn mutate_setting_value<F, R>(
        &mut self,
        setting: &StoredSetting,
        callback: F
    ) -> ()
    where
        F: FnOnce(&mut StoredSetting) -> R
    {
        match setting.setting().category().clone() {
            // categorized
            Some(category) => {
                let settings = &mut self
                    .categorized_settings_mut().write();
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
                let settings = &mut self
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
    /// If comparator returns false than setting was changed
    /// and plugin initial setting adds in changed_settings
    pub fn mutate_changed_settings<F>(
        &mut self,
        target_setting: &StoredSetting,
        plugins_store: &PluginsStore,
        comparator: F
    )
    where
        F: Fn(&SettingComponent) -> bool
    {
        let plugin = plugins_store
            .get_plugins()
            .iter()
            .find(|plugin| plugin.get_name() == target_setting.owner);

        match plugin {
            Some(plugin) => {
                let plugin_setting = plugin
                    .get_settings()
                    .iter()
                    .find(|setting|
                        setting.category() == target_setting.setting().category() &&
                        setting.title() == target_setting.setting().title()
                    );

                match plugin_setting {
                    Some(plugin_setting) => {
                        if comparator(plugin_setting.component()) {
                            if self.changed_settings.read().contains_key(target_setting.id()) {
                                let changed_settings = &mut self.changed_settings_mut().write();
                                changed_settings.remove(target_setting.id());
                                trace!("<- Returned to initial [{}]", target_setting.id())
                            }
                        } else {
                            let changed_settings = &mut self.changed_settings_mut().write();
                            changed_settings.insert(target_setting.id().clone(), plugin_setting.clone());
                            trace!("-> Changed setting [{}]", target_setting.id())
                        }
                    },
                    None => {
                        error!(
                            "Setting \"{}\" in plugin {} not found",
                            target_setting.setting().title(), plugin.get_name()
                        );
                    }
                }
            },
            None => {
                error!("Plugin with name \"{}\" not found", target_setting.owner);
            }
        }
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
