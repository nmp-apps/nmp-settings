use std::ops::Deref;

use dioxus::{logger::tracing::error, prelude::*};

use crate::components::{CheckIcon, KeyboardArrowUpIcon};

#[derive(PartialEq, Props, Clone)]
pub struct SelectProps<T: 'static + std::clone::Clone + std::cmp::PartialEq + std::fmt::Debug> {
    multiple: ReadSignal<Option<bool>>,
    items: ReadSignal<Vec<SelectItem<T>>>,
    value: ReadSignal<SelectValue<T>>,
    #[props(default = ReadSignal::new(Signal::new(false)))]
    disabled: ReadSignal<bool>,
    onclick: EventHandler<SelectValue<T>>
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

#[derive(Clone, Debug)]
pub enum SelectValue<T: std::fmt::Debug> {
    Single(Option<T>),
    Multiple(Vec<T>)
}

#[component]
pub fn Select<T>(props: SelectProps<T>) -> Element
where
    T: 'static + std::clone::Clone + std::cmp::PartialEq + std::fmt::Display + std::fmt::Debug
{
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
                    match value {
                        Some(value) => {
                            let list = props.items.read();
                            let found_item = list.iter().find(|item| item.value == value);
                            match found_item {
                                Some(item) => format!("{}", item.text),
                                None => {
                                    error!("selected value {:?} not found in select items list", value);
                                    String::from("Choose an option")
                                }   
                            }
                        },
                        None => String::from("Choose an option")
                    }
                } else {
                    // error case
                    error!("in multiple select value of single select: {:?}", value);
                    String::from("Choose an options")
                }
            },
            SelectValue::Multiple(list) => {
                if is_multiple.read().clone() {
                    let number_of_selected_values = list.len();
                    if number_of_selected_values <= 0 {
                        String::from("Choose an options")
                    } else {
                        let first_item_value = list.get(0);
                        let more_values_selected: String = if number_of_selected_values == 1 {
                            String::from("")
                        } else {
                            let without_first = number_of_selected_values - 1;
                            format!(" +{}", without_first)
                        };
                        match first_item_value {
                            Some(value) => {
                                let list = props.items.read();
                                let found_item = list.iter().find(|item| item.value == *value);
                                match found_item {
                                    Some(item) => {
                                        format!("{}{}", item.text, more_values_selected)
                                    },
                                    None => {
                                        error!("first selected value {:?} not found in select items list", value);
                                        String::from("Choose an option")
                                    }   
                                }
                                
                            },
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

    let mut is_opened = use_signal(|| false);

    let mut select_handler = move |item: SelectItem<T>| {
        let is_multiple = match *props.multiple.read() {
            Some(val) => val,
            None => false,
        };

        if is_multiple {
            match props.value.read().clone() {
                SelectValue::Multiple(list) => {
                    if list.len() <= 0 {
                        props.onclick.call(SelectValue::Multiple(vec![item.value])); // new select
                    } else {
                        if list.contains(&item.value) {
                            let iter = list.iter();
                            let filtered_list = iter.filter(|v| **v != item.value).cloned().collect();
                            props.onclick.call(SelectValue::Multiple(filtered_list));
                        } else {
                            let mut new_list = list.clone();
                            new_list.push(item.value);
                            props.onclick.call(SelectValue::Multiple(new_list));
                        }
                    }
                },
                SelectValue::Single(value) => {
                    error!("in a multiple select value of single select: {:?}", value);
                    props.onclick.call(SelectValue::Multiple(vec![]));
                }
            }
        } else {
            match props.value.read().clone() {
                SelectValue::Single(value) => {
                    match value {
                        Some(exist_value) => {
                            if exist_value == item.value {
                                props.onclick.call(SelectValue::Single(None)) // unselect
                            } else {
                                props.onclick.call(SelectValue::Single(Some(item.value))) // change
                            }
                        },
                        None => props.onclick.call(SelectValue::Single(Some(item.value))) // new select
                    }
                },
                // error case
                SelectValue::Multiple(value) => {
                    error!("in a single select value of multiple select: {:?}", value);
                    props.onclick.call(SelectValue::Single(None));
                }
            }
            is_opened.set(!is_opened())
        }
    };

    let mut item_keypress_handler = move |evt: Event<KeyboardData>, item: SelectItem<T>| {
        if evt.code() == Code::Enter || evt.code() == Code::Space {
            select_handler(item);
        }
    };

    rsx! {
        div {
            class: "ui-select",
            class: if is_opened() { "opened" } else { "" },
            class: if props.disabled.read().clone() { "disabled" },

            button {
                class: "ui-select__button",
                disabled: *props.disabled.read(),
                onclick: move |_| is_opened.set(!is_opened()),
                span { "{select_button_text}" }
                KeyboardArrowUpIcon { class: "ui-select__arrow-icon" }
            }

            ul { class: "ui-select__list",
                for item in props.items.read().clone() {
                    li {
                        class: "ui-select__item",
                        tabindex: if is_opened() { "0" } else { "-1" },
                        onclick: {
                            let item = item.clone();
                            move |_| select_handler(item.clone())
                        },
                        onkeypress: move |evt| item_keypress_handler(evt, item.clone()),
                        span { class: "ui-select__item-text", "{item.text}" }
                        if is_selected_value(item.clone(), props.value) {
                            CheckIcon { class: "ui-select__item-icon" }
                        } else {
                            div { class: "ui-select__item-icon" }
                        }
                    }
                }
            }
        }
    }
}

fn is_selected_value<T>(item: SelectItem<T>, current_value: ReadSignal<SelectValue<T>>) -> bool
where
    T: std::fmt::Debug + std::cmp::PartialEq + 'static
{
    match current_value.read().deref() {
        SelectValue::Single(value) => {
            match value {
                Some(v) => *v == item.value,
                None => false,
            }
        },
        SelectValue::Multiple(list) => list.contains(&item.value),
    }
}