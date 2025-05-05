use dioxus::prelude::*;

use crate::{components::Switch, stores::ConfigStore};

#[component]
pub fn AdvancedSettingsSwitch() -> Element {
    let mut config_store = use_context::<Signal<ConfigStore>>();

    let handler = move |event: Event<FormData>| {
        let store_mut = &mut config_store.write();
        let flag = store_mut.is_advanced_settings_mut();
        *flag = event.checked();
    };
    rsx! {
        Switch {
            title: "Enable advanced settings",
            value: config_store.read().is_advanced_settings(),
            onchange: handler,
        }
    }
}