use dioxus::prelude::*;

use crate::router::Route;
use crate::components::{LeftBar, Notifications, TopBar};


#[component]
pub fn MainLayout() -> Element {
    rsx! {
        Notifications {}

        TopBar {}

        div { class: "main-layout__wrapper",
            LeftBar {}
            div { class: "main-layout__content",
                Outlet::<Route> {}
            }
        }
    }
}
