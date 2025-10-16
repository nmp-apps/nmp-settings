use dioxus::prelude::*;
use dioxus::desktop::tao::window::ResizeDirection;
use dioxus::desktop::use_window;


#[component]
pub fn WindowResize() -> Element {
    let window = use_window();
    let window_cloned = window.clone();
    let north_resize_handler = move |_| {
        let _ = window_cloned.drag_resize_window(ResizeDirection::North);
    };

    let window_cloned = window.clone();
    let south_resize_handler = move |_| {
        let _ = window_cloned.drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::South);
    };

    let window_cloned = window.clone();
    let west_resize_handler = move |_| {
        let _ = window_cloned.drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::West);
    };

    let window_cloned = window.clone();
    let east_resize_handler = move |_| {
        let _ = window_cloned.drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::East);
    };

    let window_cloned = window.clone();
    let northwest_resize_handler = move |_| {
        let _ = window_cloned.drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::NorthWest);
    };

    let window_cloned = window.clone();
    let northeast_resize_handler = move |_| {
        let _ = window_cloned.drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::NorthEast);
    };

    let window_cloned = window.clone();
    let southeast_resize_handler = move |_| {
        let _ = window_cloned.drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::SouthEast);
    };

    let window_cloned = window.clone();
    let southwest_resize_handler = move |_| {
        let _ = window_cloned.drag_resize_window(dioxus::desktop::tao::window::ResizeDirection::SouthWest);
    };
    rsx! {
        div { class: "window-resize-north", onmousedown: north_resize_handler }
        div { class: "window-resize-south", onmousedown: south_resize_handler }
        div { class: "window-resize-west", onmousedown: west_resize_handler }
        div { class: "window-resize-east", onmousedown: east_resize_handler }

        div {
            class: "window-resize-northwest",
            onmousedown: northwest_resize_handler,
        }
        div {
            class: "window-resize-northeast",
            onmousedown: northeast_resize_handler,
        }
        div {
            class: "window-resize-southeast",
            onmousedown: southeast_resize_handler,
        }
        div {
            class: "window-resize-southwest",
            onmousedown: southwest_resize_handler,
        }
    }
}