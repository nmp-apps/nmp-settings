use dioxus::prelude::*;

use crate::get_asset;

/// Component with style links. Loads in root for prevent loading on place(as it is not web app)
#[component]
pub fn StylesLoader() -> Element {
    // common
    let reset_css: String = use_hook(|| get_asset!("/assets/styles/reset.css"));
    let theme_css: String = use_hook(|| get_asset!("/assets/styles/theme.css"));
    let main_css: String = use_hook(|| get_asset!("/assets/styles/main.css"));
    // views
    let categorized_settings: String = use_hook(|| get_asset!("/assets/styles/views/categorized_settings.css"));
    let home: String = use_hook(|| get_asset!("/assets/styles/views/home.css"));
    // app
    let left_bar: String = use_hook(|| get_asset!("/assets/styles/app/left_bar.css"));
    let notifications: String = use_hook(|| get_asset!("/assets/styles/app/notifications.css"));
    let notification: String = use_hook(|| get_asset!("/assets/styles/app/notification.css"));
    let top_bar: String = use_hook(|| get_asset!("/assets/styles/app/top_bar.css"));
    // layouts
    let main_layout: String = use_hook(|| get_asset!("/assets/styles/layouts/main_layout.css"));
    // settings
    let setting: String = use_hook(|| get_asset!("/assets/styles/settings/setting.css"));
    let settings_list: String = use_hook(|| get_asset!("/assets/styles/settings/settings_list.css"));
    // ui
    let button_group: String = use_hook(|| get_asset!("/assets/styles/ui/button_group.css"));
    let button: String = use_hook(|| get_asset!("/assets/styles/ui/button.css"));
    let icon_button: String = use_hook(|| get_asset!("/assets/styles/ui/icon_button.css"));
    let list: String = use_hook(|| get_asset!("/assets/styles/ui/list.css"));
    let number: String = use_hook(|| get_asset!("/assets/styles/ui/number.css"));
    let select: String = use_hook(|| get_asset!("/assets/styles/ui/select.css"));
    let slider: String = use_hook(|| get_asset!("/assets/styles/ui/slider.css"));
    let switch: String = use_hook(|| get_asset!("/assets/styles/ui/switch.css"));
    let text: String = use_hook(|| get_asset!("/assets/styles/ui/text.css"));

    rsx! {
        // common
        document::Stylesheet { href: "{reset_css}" }
        document::Stylesheet { href: "{theme_css}" }
        document::Stylesheet { href: "{main_css}" }
        // views
        document::Stylesheet { href: "{categorized_settings}" } // uncategorized are the same
        document::Stylesheet { href: "{home}" }
        // app
        document::Stylesheet { href: "{left_bar}" }
        document::Stylesheet { href: "{notifications}" }
        document::Stylesheet { href: "{notification}" }
        document::Stylesheet { href: "{top_bar}" }
        // layouts
        document::Stylesheet { href: "{main_layout}" }
        // settings
        document::Stylesheet { href: "{setting}" }
        document::Stylesheet { href: "{settings_list}" }
        // ui
        document::Stylesheet { href: "{button_group}" }
        document::Stylesheet { href: "{button}" }
        document::Stylesheet { href: "{icon_button}" }
        document::Stylesheet { href: "{list}" }
        document::Stylesheet { href: "{number}" }
        document::Stylesheet { href: "{select}" }
        document::Stylesheet { href: "{slider}" }
        document::Stylesheet { href: "{switch}" }
        document::Stylesheet { href: "{text}" }
    }
}