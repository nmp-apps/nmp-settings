use std::ops::Deref;

use dioxus::logger::tracing::trace;
use dioxus::prelude::*;

use crate::components::Text as UiText;
use crate::models::{Text, SettingComponent};
use crate::stores::{PluginsStore, SettingsStore, StoredSetting};

#[derive(PartialEq, Clone, Props)]
pub struct TextSettingProps {
    setting: ReadOnlySignal<StoredSetting>,
    data: ReadOnlySignal<Text>,
    disabled: ReadOnlySignal<Option<bool>>
}

#[component]
pub fn TextSetting(props: TextSettingProps) -> Element {
    let mut settings_store = use_context::<SettingsStore>();
    let plugins_store = use_context::<PluginsStore>();

    let mut handler = move |new_value: String| {
        let setting_ref = props.setting.read();
        let setting = setting_ref.deref();
        let new_value_cloned = new_value.clone();
        settings_store.mutate_setting_value(
            setting,
            |setting: &mut StoredSetting| {
                match setting.setting_mut().component_mut() {
                    SettingComponent::Text(component) => {
                        trace!("Text changed: new value is {}, old value is {}", new_value_cloned, component.value());
                        component.set_value(new_value_cloned);
                    },
                    _ => (),
                }
            }
        );
        settings_store.mutate_changed_settings(
            setting,
            &plugins_store,
            |plugin_setting_component| {
                match plugin_setting_component {
                    SettingComponent::Text(plugin_component) => {
                        *plugin_component.value() == new_value      
                    },
                    _ => false
                }
            }
        );
    };

    rsx! {
        UiText {
            oninput: move |new_value| handler(new_value),
            value: props.data.read().value(),
            min_length: props.data.read().min_length(),
            max_length: props.data.read().max_length(),
            disabled: match *props.disabled.read() {
                Some(v) => v,
                None => false,
            },
        }
    }
}