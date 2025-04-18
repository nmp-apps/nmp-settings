use dioxus::{logger::tracing::error, prelude::*};

use crate::get_asset;

#[derive(PartialEq, Props, Clone)]
pub struct SelectProps<T: 'static + std::clone::Clone + std::cmp::PartialEq + std::fmt::Debug> {
    multiple: ReadOnlySignal<Option<bool>>,
    items: Vec<SelectItem<T>>,
    value: ReadOnlySignal<SelectValue<T>>, // todo: add ReadOnlySignal<Option<SelectValue<T>>>
}

#[derive(PartialEq, Clone)]
pub struct SelectItem<T> {
    text: String,
    value: T
}

impl<T> SelectItem<T> {
    pub fn new(text: String, value: T) -> SelectItem<T> {
        SelectItem { text, value }
    }
}

#[derive(Clone)]
pub enum SelectValue<T: std::fmt::Debug> {
    Single(T),
    Multiple(Vec<T>)
}

#[component]
pub fn Select<T: 'static + std::clone::Clone + std::cmp::PartialEq + std::fmt::Display + std::fmt::Debug>(props: SelectProps<T>) -> Element {
    let styles: String = use_hook(|| get_asset!("/assets/styles/ui/select.css"));
    let is_multiple = use_memo(move || {
        match *props.multiple.read() {
            Some(value) => value,
            None => false
        }
    });

    let select_button_text = use_memo(move || {
        match props.value.read().clone() {
            SelectValue::Single(value) => {
                if !is_multiple.read().clone() {
                    format!("{value}")
                } else {
                    // error case
                    error!("in multiple select value of single select: {value}");
                    String::from("Choose an options")
                }
            },
            SelectValue::Multiple(list) => {
                if is_multiple.read().clone() {
                    let length = list.len();
                    if length <= 0 {
                        String::from("Choose an options")
                    } else {
                        let first_item_text = list.get(0);
                        let values_without_first: String = if length == 1 {
                            String::from("")
                        } else {
                            let without_first = length - 1;
                            without_first.to_string()
                        };
                        match first_item_text {
                            Some(text) => format!("{text} +{values_without_first}"),
                            None => {
                                error!("in multiple select can't get the first item in the selected values.");
                                String::from("Choose an options")
                            }
                        }
                    }
                } else {
                    //error case
                    error!("in a single select, the value of a multiple select: {:?}", list);
                    String::from("Choose an option")
                }
            }
        }
    });

    rsx! {
        div { class: "ui-select",
            document::Stylesheet { href: "{styles}" }

            button { class: "ui-select__button", "{select_button_text}" }

            ul { class: "ui-select__list",
                for item in props.items {
                    li { class: "ui-select__item", "{item.text}-{item.value}" }
                }
            }
        }
    }
}