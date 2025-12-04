use dioxus::desktop::{use_window, use_wry_event_handler};
use dioxus::desktop::tao::event::{Event as WryEvent, WindowEvent};
use dioxus::desktop::wry::dpi::PhysicalSize;
use dioxus::prelude::*;

pub fn use_window_size() -> Signal<(u32, u32)> {
    let window = use_window();
    let scale_factor = window.scale_factor();
    let size = use_signal(|| (0u32, 0u32));

    use_wry_event_handler({
        let mut size = size.clone();
        move |event, _| {
            if let WryEvent::WindowEvent {
                event: WindowEvent::Resized(
                    PhysicalSize { width, height }
                ),
                ..
            } = event {
                let width = width.clone() as f64 / scale_factor;
                let height = height.clone() as f64 / scale_factor;
                size.set((width as u32, height as u32));
            };
        }
    });

    size
}