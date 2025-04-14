use std::collections::HashMap;

use dioxus::hooks::use_context_provider;

use crate::models::{Settings, SettingsCategory};

/// Settings of plugins
#[derive(Clone)]
struct SettingsStore {
    /// Settings by user friendly category
    categorrized_settings: HashMap<SettingsCategory, Vec<StoredSetting>>,
    /// Settings by owner
    uncategorized_settings: HashMap<String, Vec<StoredSetting>>,
}

#[derive(Clone)]
struct StoredSetting {
    owner: String,
    settings: Settings,
}

pub fn use_settings_store(initialState: SettingsStore) {
    use_context_provider(|| initialState);
}
