use dioxus::prelude::*;

use crate::components::{List, ListItem};
use crate::models::SettingsCategory;

const STYLES: Asset = asset!("/assets/styles/app/left_bar.css");

#[component]
pub fn LeftBar() -> Element {
    const CATEGORIES: [SettingsCategory; 10] = [SettingsCategory::Appearance, SettingsCategory::Bluetooth, SettingsCategory::Desktop, SettingsCategory::Display, SettingsCategory::LockScreen, SettingsCategory::Network, SettingsCategory::Notifications, SettingsCategory::Security, SettingsCategory::Sound, SettingsCategory::Users];
    let list_items: Vec<ListItem<String>> = CATEGORIES.map(|category|
        ListItem::new(
            category.get_name(),
            category.get_name(),
            category.get_icon()
        )
    ).to_vec();
    rsx! {
        div { class: "left-bar",

            document::Link { rel: "stylesheet", href: STYLES }
            List::<String> { items: list_items }
        }
    }
}