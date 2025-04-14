use std::process::Command;
use serde::{Deserialize, Serialize};
use dioxus::logger::tracing::error;

use super::SettingsCategory;

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Plugin {
    plugin_version: String,
    nmp_settings_version: String,
    plugin_name: String,
    settings: Vec<Settings>,
}

impl Plugin {
    // pub fn new(plugin_version: String, nmp_settings_version: String, plugin_name: String, settings: Vec<Settings>,) -> Plugin {
    //     Plugin {
    //         plugin_version,
    //         nmp_settings_version,
    //         plugin_name,
    //         settings,
    //     }
    // }
    pub fn from_command(command: &str) -> Result<Plugin, &str> {
        let mut command = Command::new(command);
        let stdout = match command.arg("--json").output() {
            Ok(v) => v.stdout,
            Err(_) => {
                error!("Can't get plugin output data");
                return Err("Can't get plugin output data")
            }
        };
        let json = match str::from_utf8(&stdout) {
            Ok(v) => v,
            Err(_) => {
                error!("Can't read plugin output data");
                return Err("Can't read plugin output data")
            },
        };
        let plugin: Plugin = match serde_json::from_str(json) {
            Ok(v) => v,
            Err(e) => {
                error!("Parsing plugin data error, {}", e);
                return Err("Parsing plugin data error")
            },
        };
        Ok(plugin)
    }
    fn check_version(&self) -> bool {
        true
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Settings {
    Switch(Switch),
    Text(Text),
    Number(Number),
    ButtonGroup(ButtonGroup),
    Select(Select),
    MultiSelect(MultiSelect),
    Slider(Slider)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SettingCommon {
    category: Option<SettingsCategory>,
    title: String,
    description: String,
    confirmation: Option<ConfirmationWindow>,
    is_advanced: bool,
}

// impl SettingCommon {
//     pub fn new(category: Option<SettingsCategory>, title: String, description: String, confirmation: Option<ConfirmationWindow>, is_advanced: bool,) -> SettingCommon {
//         SettingCommon { category, title, description, confirmation, is_advanced, }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConfirmationWindow {
    title: String,
    description: String,
}

// impl ConfirmationWindow {
//     pub fn new(title: String, description: String) -> ConfirmationWindow {
//         ConfirmationWindow { title, description }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Switch {
    setting_common: SettingCommon,
    value: bool
}

// impl Switch {
//     pub fn new(setting_common: SettingCommon, value: bool) -> Switch {
//         Switch { setting_common, value }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    setting_common: SettingCommon,
    value: String,
    min_length: u32,
    max_length: u32
}

// impl Text {
//     pub fn new(setting_common: SettingCommon, value: String, min_length: u32, max_length: u32) -> Text {
//         Text { setting_common, value, min_length, max_length }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Number {
    setting_common: SettingCommon,
    value: f64,
    min: f64,
    max: f64,
    step: f64,
}

// impl Number {
//     pub fn new(setting_common: SettingCommon, value: f64, min: f64, max: f64, step: f64) -> Number {
//         Number { setting_common, value, min, max, step }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ButtonGroup {
    setting_common: SettingCommon,
    value: String,
    items: Vec<ButtonGroupItem>
}

// impl ButtonGroup {
//     pub fn new(setting_common: SettingCommon, value: String, items: Vec<ButtonGroupItem>) -> ButtonGroup {
//         ButtonGroup { setting_common, value, items }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ButtonGroupItem {
    text: String,
    value: String,
}

// impl ButtonGroupItem {
//     pub fn new(text: String, value: String) -> ButtonGroupItem {
//         ButtonGroupItem { text, value }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Select {
    setting_common: SettingCommon,
    value: String,
    items: Vec<SelectItem>
}

// impl Select {
//     pub fn new(setting_common: SettingCommon, value: String, items: Vec<SelectItem>) -> Select {
//         Select { setting_common, value, items }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SelectItem {
    text: String,
    value: String
}

// impl SelectItem {
//     pub fn new(text: String, value: String) -> SelectItem {
//         SelectItem { text, value }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MultiSelect {
    setting_common: SettingCommon,
    value: Vec<String>,
    items: Vec<MultiSelectItem>
}

// impl MultiSelect {
//     pub fn new(setting_common: SettingCommon, value: Vec<String>, items: Vec<MultiSelectItem>) -> MultiSelect {
//         MultiSelect { setting_common, value, items }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MultiSelectItem {
    text: String,
    value: String
}

// impl MultiSelectItem {
//     pub fn new(text: String, value: String) -> MultiSelectItem {
//         MultiSelectItem { text, value }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Slider {
    setting_common: SettingCommon,
    value: f64,
    min: f64,
    max: f64,
    step: f64,
}

// impl Slider {
//     pub fn new(setting_common: SettingCommon, value: f64, min: f64, max: f64, step: f64) -> Slider {
//         Slider { setting_common, value, min, max, step }
//     }
// }