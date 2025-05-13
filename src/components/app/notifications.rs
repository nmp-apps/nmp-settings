use dioxus::prelude::*;

use crate::stores::AppStore;
use crate::models::{
    Icon,
    Notification as NotificationModel
};
use crate::components::IconButton;

#[component]
pub fn Notifications() -> Element {
    let mut app_store = use_context::<Signal<AppStore>>();

    rsx! {
        ul { class: "app-notifications",

            for notification in app_store.read().notifications().clone() {
                li {
                    Notification {
                        data: notification.clone(),
                        onclose: move |()| app_store.write().remove_notification(&notification),
                    }
                }
            }
        }
    }
}

#[derive(PartialEq, Props, Clone)]
struct NotificationProps {
    data: NotificationModel,
    onclose: Option<Callback>
}

#[component]
fn Notification(props: NotificationProps) -> Element {

    rsx! {
        div { class: "notification",
            div { class: "notification__first-line",
                h1 { class: "notification__title", {format!("{}", props.data.title())} }
                if !props.data.permanent() {
                    IconButton {
                        icon: Icon::Close,
                        size: 16,
                        onclick: move |_| match props.onclose {
                            Some(cb) => cb.call(()),
                            None => {}
                        },
                    }
                }
            }
            if let Some(text) = props.data.description() {
                p { class: "notification__description", {text.clone()} }
            }
        }
    }
}