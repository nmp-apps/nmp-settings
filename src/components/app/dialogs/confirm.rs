use dioxus::desktop::window;
use dioxus::prelude::*;

use crate::get_asset;
use crate::components::Button;
use crate::stores::{AppStore, AppWindowName};

#[derive(PartialEq, Props, Clone)]
pub struct ConfirmProps {
    title: String,
    description: Option<String>,
    app_store: Signal<AppStore>,
    app_window_name: AppWindowName,
    onconfirm: EventHandler<bool>
}

#[component]
pub fn Confirm(mut props: ConfirmProps) -> Element {
    let styles: String = use_hook(|| get_asset!("/assets/styles/app/dialogs/confirm.css"));

    let handler = move |answer: bool| {
        props.onconfirm.call(answer);
        println!("{:#?}", props.app_window_name.clone());
        props.app_store.write().remove_opened_window_by_name(props.app_window_name.clone());
        window().close();
    };

    rsx! {
        div { class: "confirm",
            document::Stylesheet { href: "{styles}" }

            h1 { class: "confirm__title", {props.title} }
            if let Some(description) = props.description {
                p { class: "confirm__description", {description} }
            }
            div { class: "confirm__actions",
                Button {
                    onclick: {
                        let mut handler = handler.clone();
                        move |_| handler(false)
                    },
                    "No"
                }
                Button {
                    onclick: {
                        let mut handler = handler.clone();
                        move |_| handler(true)
                    },
                    "Yes"
                }
            }
        }
    }
}