use dioxus::{document, prelude::*};

use crate::get_asset;
use crate::stores::StoredSetting;

#[derive(PartialEq, Props, Clone)]
pub struct SettingProps {
    setting: StoredSetting
}

/// Main Setting component
#[component]
pub fn Setting(props: SettingProps) -> Element {
    let styles: String = use_hook(|| get_asset!("/assets/styles/settings/setting.css"));

    rsx! {
        div { class: "setting",
            document::Stylesheet { href: "{styles}" }

            div { class: "setting__content",

                h6 { {format!("{}", props.setting.setting().title())} }
                if props.setting.setting().description().len() > 0 {
                    p { {format!("{}", props.setting.setting().description())} }
                }
            }

            div { class: "setting__actions",
                p { {format!("{:?}", props.setting.setting().component())} }
            }
        }
    }
}