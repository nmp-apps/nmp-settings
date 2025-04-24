use std::ops::Deref;

use dioxus::logger::tracing::trace;
use dioxus::prelude::*;

use crate::components::Slider as UiSlider;
use crate::models::{Slider, SettingComponent};
use crate::stores::{PluginsStore, SettingsStore, StoredSetting};

#[derive(PartialEq, Clone, Props)]
pub struct SliderSettingProps {
    setting: ReadOnlySignal<StoredSetting>,
    data: ReadOnlySignal<Slider>
}

#[component]
pub fn SliderSetting(props: SliderSettingProps) -> Element {
    let mut settings_store = use_context::<SettingsStore>();
    let plugins_store = use_context::<PluginsStore>();

    let mut handler = move |new_value: f64| {
        let setting_ref = props.setting.read();
        let setting = setting_ref.deref();
        settings_store.mutate_setting_value(
            setting,
            |setting: &mut StoredSetting| {
                match setting.setting_mut().component_mut() {
                    SettingComponent::Slider(component) => {
                        trace!("Slider changed: new value is {}, old value is {}", new_value, component.value());
                        component.set_value(new_value);
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
                    SettingComponent::Slider(plugin_component) => {
                        plugin_component.value() == new_value      
                    },
                    _ => false
                }
            }
        );
    };

    rsx! {
        UiSlider {
            oninput: move |new_value| handler(new_value),
            min: props.data.read().min(),
            max: props.data.read().max(),
            step: props.data.read().step(),
            value: props.data.read().value(),
        }
    }
}