use dioxus::prelude::*;

use crate::components::{List, ListItem};
use crate::models::SettingsCategory;
use crate::router::Route;

#[component]
pub fn CategoryNavigation() -> Element {
    let nav = navigator();

    const CATEGORIES: [SettingsCategory; 10] = [SettingsCategory::Appearance, SettingsCategory::Bluetooth, SettingsCategory::Desktop, SettingsCategory::Display, SettingsCategory::LockScreen, SettingsCategory::Network, SettingsCategory::Notifications, SettingsCategory::Security, SettingsCategory::Sound, SettingsCategory::Users];
    let list_items: Vec<ListItem<String>> = CATEGORIES.map(|category|
        ListItem::new(
            category.get_name(),
            category.get_name(),
            category.get_icon()
        )
    ).to_vec();

    let category_click_handler = move |list_item: ListItem<String>| {
        nav.push(Route::CategorizedSettings { category_name: list_item.get_value().clone() });
    };

    rsx! {
        List::<String> { items: list_items, onclick: category_click_handler }
    }
}