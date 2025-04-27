use dioxus::{desktop::{tao::window::WindowId, window}, prelude::*};

use crate::{components::Button, get_asset};

#[derive(PartialEq, Props, Clone)]
pub struct ClosingAppProps {
    parent_window_id: ReadOnlySignal<WindowId>
}

#[component]
pub fn ClosingApp(props: ClosingAppProps) -> Element {
    let reset_css: String = use_hook(|| get_asset!("/assets/styles/app/dialogs/closing_app.css"));

    let cancel = move |_| {
        window().close();
    };

    let close = move |_| {
        window().close_window(*props.parent_window_id.read());
        window().close();
    };

    rsx! {
        div { class: "closing-app",
            document::Stylesheet { href: "{reset_css}", rel: "preload" }

            h1 { class: "closing-app__title",
                "You have unsaved changes. Are you sure you want to exit?"
            }
            div { class: "closing-app__actions",
                Button { onclick: cancel, "No" }
                Button { onclick: close, "Yes" }
            }
        }
    }
}