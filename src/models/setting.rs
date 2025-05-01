use serde::{Deserialize, Serialize};

use super::SettingsCategory;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Setting {
    category: Option<SettingsCategory>,
    title: String,
    description: String,
    confirmation: Option<ConfirmationWindow>, // TODO
    is_advanced: bool,
    component: SettingComponent,
}

impl Setting {
    pub fn category(&self) -> &Option<SettingsCategory> {
        &self.category
    }
    pub fn title(&self) -> &String {
        &self.title
    }
    pub fn description(&self) -> &String {
        &self.description
    }
    pub fn is_advanced(&self) -> bool {
        self.is_advanced
    }
    pub fn component(&self) -> &SettingComponent {
        &self.component
    }
    pub fn component_mut(&mut self) -> &mut SettingComponent {
        &mut self.component
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum SettingComponent {
    Switch(Switch),
    Text(Text),
    Number(Number),
    ButtonGroup(ButtonGroup),
    Select(Select),
    MultiSelect(MultiSelect),
    Slider(Slider)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ConfirmationWindow {
    title: String,
    description: String,
}

// impl ConfirmationWindow {
//     pub fn new(title: String, description: String) -> ConfirmationWindow {
//         ConfirmationWindow { title, description }
//     }
// }

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Switch {
    value: bool
}

impl Switch {
    // pub fn new(value: bool) -> Switch {
    //     Switch { value }
    // }
    pub fn value(&self) -> bool {
        self.value
    }
    pub fn set_value(&mut self, new_value: bool) {
        self.value = new_value;
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    value: String,
    min_length: u32,
    max_length: u32
}

impl Text {
    // pub fn new(setting_common: SettingCommon, value: String, min_length: u32, max_length: u32) -> Text {
    //     Text { setting_common, value, min_length, max_length }
    // }
    pub fn value(&self) -> &String {
        &self.value
    }
    pub fn set_value(&mut self, new_value: String) {
        self.value = new_value
    }
    pub fn min_length(&self) -> u32 {
        self.min_length
    }
    pub fn max_length(&self) -> u32 {
        self.max_length
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Number {
    value: f64,
    min: f64,
    max: f64,
    step: f64,
}

impl Number {
    // pub fn new(setting_common: SettingCommon, value: f64, min: f64, max: f64, step: f64) -> Number {
    //     Number { setting_common, value, min, max, step }
    // }
    pub fn value(&self) -> f64 {
        self.value
    }
    pub fn set_value(&mut self, new_value: f64) {
        self.value = new_value
    }
    pub fn min(&self) -> f64 {
        self.min
    }
    pub fn max(&self) -> f64 {
        self.max
    }
    pub fn step(&self) -> f64 {
        self.step
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ButtonGroup {
    value: String,
    items: Vec<ButtonGroupItem>
}

impl ButtonGroup {
    // pub fn new(setting_common: SettingCommon, value: String, items: Vec<ButtonGroupItem>) -> ButtonGroup {
    //     ButtonGroup { setting_common, value, items }
    // }
    pub fn value(&self) -> &String {
        &self.value
    }
    pub fn set_value(&mut self, new_value: String) {
        self.value = new_value;
    }
    pub fn items(&self) -> &Vec<ButtonGroupItem> {
        &self.items
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ButtonGroupItem {
    text: String,
    value: String,
}

impl ButtonGroupItem {
    // pub fn new(text: String, value: String) -> ButtonGroupItem {
    //     ButtonGroupItem { text, value }
    // }
    pub fn text(&self) -> &String {
        &self.text
    }
    pub fn value(&self) -> &String {
        &self.value
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Select {
    value: Option<String>,
    items: Vec<SelectItem>
}

impl Select {
    // pub fn new(setting_common: SettingCommon, value: String, items: Vec<SelectItem>) -> Select {
    //     Select { setting_common, value, items }
    // }
    pub fn value(&self) -> &Option<String> {
        &self.value
    }
    pub fn set_value(&mut self, new_value: Option<String>) {
        self.value = new_value;
    }
    pub fn items(&self) -> &Vec<SelectItem> {
        &self.items
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SelectItem {
    text: String,
    value: String
}

impl SelectItem {
    // pub fn new(text: String, value: String) -> SelectItem {
    //     SelectItem { text, value }
    // }
    pub fn text(&self) -> &String {
        &self.text
    }
    pub fn value(&self) -> &String {
        &self.value
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MultiSelect {
    value: Vec<String>,
    items: Vec<MultiSelectItem>
}

impl MultiSelect {
    // pub fn new(setting_common: SettingCommon, value: Vec<String>, items: Vec<MultiSelectItem>) -> MultiSelect {
    //     MultiSelect { setting_common, value, items }
    // }
    pub fn value(&self) -> &Vec<String> {
        &self.value
    }
    pub fn set_value(&mut self, new_value: Vec<String>) {
        self.value = new_value;
    }
    pub fn items(&self) -> &Vec<MultiSelectItem> {
        &self.items
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MultiSelectItem {
    text: String,
    value: String
}

impl MultiSelectItem {
    // pub fn new(text: String, value: String) -> MultiSelectItem {
    //     MultiSelectItem { text, value }
    // }
    pub fn text(&self) -> &String {
        &self.text
    }
    pub fn value(&self) -> &String {
        &self.value
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Slider {
    value: f64,
    min: f64,
    max: f64,
    step: f64,
}

impl Slider {
    // pub fn new(setting_common: SettingCommon, value: f64, min: f64, max: f64, step: f64) -> Slider {
    //     Slider { setting_common, value, min, max, step }
    // }
    pub fn value(&self) -> f64 {
        self.value
    }
    pub fn set_value(&mut self, new_value: f64) {
        self.value = new_value
    }
    pub fn min(&self) -> f64 {
        self.min
    }
    pub fn max(&self) -> f64 {
        self.max
    }
    pub fn step(&self) -> f64 {
        self.step
    }
}
