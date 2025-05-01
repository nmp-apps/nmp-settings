use dioxus::prelude::*;

use crate::{components::Switch, stores::AppStore};

#[component]
pub fn AdvancedSettingsSwitch() -> Element {
    let mut app_store = use_context::<Signal<AppStore>>();

    let handler = move |event: Event<FormData>| {
        let store_mut = &mut app_store.write();
        let flag = store_mut.is_advanced_settings_mut();
        *flag = event.checked();
    };
    rsx! {
        Switch {
            title: "Enable advanced settings",
            value: app_store.read().is_advanced_settings(),
            onchange: handler,
        }
    }
}