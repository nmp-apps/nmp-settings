use dioxus::desktop::window;
use dioxus::prelude::*;

use crate::get_asset;
use crate::components::Button;

#[derive(PartialEq, Props, Clone)]
pub struct ConfirmProps {
    title: String,
    description: Option<String>,
    onconfirm: EventHandler<bool>
}

#[component]
pub fn Confirm(props: ConfirmProps) -> Element {
    let styles: String = use_hook(|| get_asset!("/assets/styles/app/dialogs/confirm.css"));

    let handler = move |answer: bool| {
        props.onconfirm.call(answer);
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
                Button { onclick: move |_| handler(false), "No" }
                Button { onclick: move |_| handler(true), "Yes" }
            }
        }
    }
}