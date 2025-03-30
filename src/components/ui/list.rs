use std::fmt::Display;
use std::cmp::PartialEq;

use dioxus::prelude::*;

use crate::models::Icon;

const STYLES: Asset = asset!("/assets/styles/ui/list.css");

#[derive(PartialEq, Clone)]
pub struct ListItem<T> {
    name: String,
    value: T,
    icon: Icon
}

impl<T> ListItem<T> {
    pub fn new(name: String, value: T, icon: Icon) -> ListItem<T> {
        ListItem { name, value, icon }
    }
}

#[derive(PartialEq, Props, Clone)]
pub struct ListProps<T: 'static + PartialEq> {
    items: Vec<ListItem<T>>
}

#[component]
pub fn List<T: Display + PartialEq>(props: ListProps<T>) -> Element {
    rsx! {
        ul { class: "ui-list",
            document::Link { rel: "stylesheet", href: STYLES }

            for item in &props.items {
                li { class: "ui-list__item",
                    {item.icon.to_component(String::from("20px"))}
                    "{item.name}"
                }
            }
        }
    }
}