use dioxus::prelude::*;

// common
static RESET_CSS: Asset = asset!("/assets/styles/reset.css");
static THEME_CSS: Asset = asset!("/assets/styles/theme.css");
static MAIN_CSS: Asset = asset!("/assets/styles/main.css");
// ui
static BUTTON_GROUP_CSS: Asset = asset!("/assets/styles/ui/button_group.css");
static BUTTON_CSS: Asset = asset!("/assets/styles/ui/button.css");
static ICON_BUTTON_CSS: Asset = asset!("/assets/styles/ui/icon_button.css");
static LIST_CSS: Asset = asset!("/assets/styles/ui/list.css");
static NUMBER_CSS: Asset = asset!("/assets/styles/ui/number.css");
static SELECT_CSS: Asset = asset!("/assets/styles/ui/select.css");
static SLIDER_CSS: Asset = asset!("/assets/styles/ui/slider.css");
static SWITCH_CSS: Asset = asset!("/assets/styles/ui/switch.css");
static TEXT_CSS: Asset = asset!("/assets/styles/ui/text.css");
// views
static CATEGORIZED_SETTINGS_CSS: Asset = asset!("/assets/styles/views/categorized_settings.css");
static HOME_CSS: Asset = asset!("/assets/styles/views/home.css");
// app
static DIALOG_WINDOW_WRAPPER_CSS: Asset = asset!("/assets/styles/app/dialogs/window_wrapper.css");
static LEFT_BAR_CSS: Asset = asset!("/assets/styles/app/left_bar.css");
static NOTIFICATIONS_CSS: Asset = asset!("/assets/styles/app/notifications.css");
static NOTIFICATION_CSS: Asset = asset!("/assets/styles/app/notification.css");
static TOP_BAR_CSS: Asset = asset!("/assets/styles/app/top_bar.css");
// dialogs
static WINDOW_BAR_CSS: Asset = asset!("/assets/styles/app/dialogs/window_bar.css");
// layouts
static MAIN_LAYOUT_CSS: Asset = asset!("/assets/styles/layouts/main_layout.css");
// settings
static SETTINGS_CSS: Asset = asset!("/assets/styles/settings/setting.css");
static SETTINGS_LIST_CSS: Asset = asset!("/assets/styles/settings/settings_list.css");

/// Component with style links. Loads in root for prevent loading on place(as it is not web app)
#[component]
pub fn StylesLoader() -> Element {
    rsx! {
        // Styles connected in strict order because of css hierarchy (main first, then ui and etc.)
        // common
        document::Link { href: RESET_CSS, rel: "stylesheet" }
        document::Link { href: THEME_CSS, rel: "stylesheet" }
        document::Link { href: MAIN_CSS, rel: "stylesheet" }
        // ui
        document::Stylesheet { href: BUTTON_GROUP_CSS, rel: "stylesheet" }
        document::Stylesheet { href: BUTTON_CSS, rel: "stylesheet" }
        document::Stylesheet { href: ICON_BUTTON_CSS, rel: "stylesheet" }
        document::Stylesheet { href: LIST_CSS, rel: "stylesheet" }
        document::Stylesheet { href: NUMBER_CSS, rel: "stylesheet" }
        document::Stylesheet { href: SELECT_CSS, rel: "stylesheet" }
        document::Stylesheet { href: SLIDER_CSS, rel: "stylesheet" }
        document::Stylesheet { href: SWITCH_CSS, rel: "stylesheet" }
        document::Stylesheet { href: TEXT_CSS, rel: "stylesheet" }
        // views
        document::Stylesheet { href: CATEGORIZED_SETTINGS_CSS, rel: "stylesheet" }
        document::Stylesheet { href: HOME_CSS, rel: "stylesheet" }
        // app
        document::Stylesheet { href: DIALOG_WINDOW_WRAPPER_CSS, rel: "stylesheet" }
        document::Stylesheet { href: LEFT_BAR_CSS, rel: "stylesheet" }
        document::Stylesheet { href: NOTIFICATIONS_CSS, rel: "stylesheet" }
        document::Stylesheet { href: NOTIFICATION_CSS, rel: "stylesheet" }
        document::Stylesheet { href: TOP_BAR_CSS, rel: "stylesheet" }
        // dialogs
        document::Stylesheet { href: WINDOW_BAR_CSS, rel: "stylesheet" }
        // layouts
        document::Stylesheet { href: MAIN_LAYOUT_CSS, rel: "stylesheet" }
        // settings
        document::Stylesheet { href: SETTINGS_CSS, rel: "stylesheet" }
        document::Stylesheet { href: SETTINGS_LIST_CSS, rel: "stylesheet" }
    }
}