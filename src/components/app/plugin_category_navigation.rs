use std::ops::Deref;

use dioxus::prelude::*;

use crate::components::{List, ListItem};
use crate::router::Route;
use crate::stores::SettingsStore;

#[component]
pub fn PluginCategoryNavigation() -> Element {
    let nav = navigator();
    let settings_store = use_context::<SettingsStore>();

    let items = use_memo(move || {
        let uncategorized_settings = settings_store.uncategorized_settings();
        let uncategorized_settings = uncategorized_settings.read();
        let plugin_names: Vec<String> = uncategorized_settings.keys().cloned().collect();
        plugin_names.iter().cloned().map(|name|
            ListItem::new(
                name.clone(),
                name,
                None
            )
        ).collect()
    });

    let category_click_handler = move |item: ListItem<String>| {
        nav.push(Route::UncategorizedSettings { plugin_name: item.get_value().clone() });
    };

    rsx! {
        List::<String> { items: items(), onclick: category_click_handler }
    }
}