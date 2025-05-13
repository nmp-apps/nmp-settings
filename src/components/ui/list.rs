use std::fmt::Display;
use std::cmp::PartialEq;

use dioxus::prelude::*;

use crate::models::Icon;

#[derive(PartialEq, Clone)]
pub struct ListItem<T> {
    name: String,
    value: T,
    icon: Option<Icon>
}

impl<T> ListItem<T> {
    pub fn new(name: String, value: T, icon: Option<Icon>) -> ListItem<T> {
        ListItem { name, value, icon }
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct ListProps<T: 'static + PartialEq + Clone> {
    capitalized: ReadOnlySignal<Option<bool>>,
    items: Vec<ListItem<T>>,
    onclick: Option<EventHandler<ListItem<T>>>
}

#[component]
pub fn List<T: Display + PartialEq + Clone>(props: ListProps<T>) -> Element {
    rsx! {
        ul { class: "ui-list",
            for item in props.items {
                ListItem {
                    item,
                    capitalized: props.capitalized,
                    onclick: props.onclick.clone(),
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct ListItemProps<T: 'static + PartialEq + Clone> {
    capitalized: ReadOnlySignal<Option<bool>>,
    item: ListItem<T>,
    onclick: Option<EventHandler<ListItem<T>>>
}

#[component]
fn ListItem<T: Display + PartialEq + Clone>(props: ListItemProps<T>) -> Element {
    let item = props.item.clone();
    let capitalized = use_memo(move || props.capitalized.read().unwrap_or(false));
    rsx! {
        li {
            class: "ui-list__item",
            class: if capitalized() { "capitalized" } else { "" },
            onclick: move |_| {
                if let Some(handler) = props.onclick {
                    handler.call(props.item.clone())
                }
            },
            key: item.name.clone(),

            match item.icon {
                Some(icon) => icon.to_component(String::from("20px")),
                None => {
                    rsx! {}
                }
            }
            "{item.name}"
        }
    }
}