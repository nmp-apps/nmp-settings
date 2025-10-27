use dioxus::{desktop::{tao::window::WindowId, use_window}, prelude::*};

use crate::components::Button;

static STYLES: Asset = asset!("/assets/styles/app/dialogs/closing_app.css");

#[derive(PartialEq, Props, Clone)]
pub struct ClosingAppProps {
    parent_window_id: ReadSignal<WindowId>,
    close_windows: Callback,
    close_self: Callback
}

#[component]
pub fn ClosingApp(props: ClosingAppProps) -> Element {
    let window = use_window();

    let cancel = move || {
        props.close_self.call(())
    };

    let close_windows = props.close_windows.clone();
    let window = window.clone();
    let close = move |_| {
        close_windows(());
        window.close_window(*props.parent_window_id.read());
    };

    rsx! {
        div { class: "closing-app",
            document::Stylesheet { href: "{STYLES}", rel: "preload" }

            h1 { class: "closing-app__title",
                "You have unsaved changes."
                br {}
                "Are you sure you want to exit?"
            }
            div { class: "closing-app__actions",
                Button { onclick: move |_| cancel(), "No" }
                Button { onclick: close, "Yes" }
            }
        }
    }
}