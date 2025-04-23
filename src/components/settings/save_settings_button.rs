use dioxus::prelude::*;

use crate::stores::SettingsStore;
use crate::components::Button;

#[component]
pub fn SaveSettingsButton() -> Element {
    let settings_store = use_context::<SettingsStore>();
    let is_disabled = use_memo(move || {
        settings_store.changed_settings().read().len() <= 0
    });

    let handler = move |_| {
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