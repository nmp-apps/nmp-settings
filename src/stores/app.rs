use std::fs;
use serde::{Serialize, Deserialize};

use dioxus::prelude::*;
use dioxus::logger::tracing::{error, info, trace};

use crate::utils::get_app_config_path;

/// User app settings store.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppStore {
    is_advanced_settings: bool
}

impl AppStore {
    pub fn new(is_advanced_settings: bool) -> AppStore {
        AppStore { is_advanced_settings }
    }
    pub fn is_advanced_settings(&self) -> bool {
        self.is_advanced_settings
    }
    pub fn is_advanced_settings_mut(&mut self) -> &mut bool {
        &mut self.is_advanced_settings
    }
    pub fn store(&self) -> Result<(), &str> {
        let path = get_app_config_path().expect("Failed to get app config path");
        let serialized_store = match serde_json::to_string_pretty(self) {
            Ok(json) => { json },
            Err(err) => {
                error!("Error while trying to serialize app settings: {}", err);
                return Err("Error while trying to serialize app settings");
            }
        };
        match fs::write(&path, serialized_store) {
            Ok(_) => {
                info!("Saved config to {}", path.display());
                Ok(())
            },
            Err(err) => {
                error!("Failed on saving app settings: {}", err);
                Err("Failed on saving app settings")
            }
        }
    }
    // Load config from user directory
    pub fn from_config() -> Result<AppStore, ()> {
        let config_path = get_app_config_path();
        match config_path {
            Some(path) => {
                let json = match fs::read_to_string(&path) {
                    Ok(string) => string,
                    Err(err) => {
                        error!("Failed reading json from path {}: {}", path.display(), err);
                        return Err(());
                    }
                };
                let config: AppStore = match serde_json::from_str(&json) {
                    Ok(app_store) => app_store,
                    Err(err) => {
                        error!("Failed parsing config JSON: {}", err);
                        return Err(());
                    }
                };
                Ok(config)
            },
            None => {
                error!("Failed to get config path");
                Err(())
            },
        }
    }
}

pub fn use_app_store() {
    trace!("Init app store...");


    use_context_provider(|| Signal::new(AppStore::from_config().unwrap_or(
        AppStore::new(
            false,
        )
    )));
    let app_store = use_context::<Signal<AppStore>>();

    // update persistent store in ~/.config/nmp/settings/config.json
    use_effect(move || {
        let _ = app_store.read().store();
    });
}
