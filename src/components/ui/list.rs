use std::fmt::Display;
use std::cmp::PartialEq;

use dioxus::prelude::*;

use crate::get_asset;
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
    items: Vec<ListItem<T>>,
    onclick: Option<EventHandler<ListItem<T>>>
}

#[component]
pub fn List<T: Display + PartialEq + Clone>(props: ListProps<T>) -> Element {
    let styles: String = use_hook(|| get_asset!("/assets/styles/ui/list.css"));
    rsx! {
        ul { class: "ui-list",
            document::Link { rel: "stylesheet", href: "{styles}" }

            for item in props.items {
                ListItem { item, onclick: props.onclick.clone() }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct ListItemProps<T: 'static + PartialEq + Clone> {
    item: ListItem<T>,
    onclick: Option<EventHandler<ListItem<T>>>
}

#[component]
fn ListItem<T: Display + PartialEq + Clone>(props: ListItemProps<T>) -> Element {
    let item = props.item.clone();
    rsx! {
        li {
            class: "ui-list__item",
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