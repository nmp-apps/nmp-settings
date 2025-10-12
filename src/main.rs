use constants::IS_WINDOW_CONTEXT_MENU_DISABLED;
// use std::path::Path;
use dioxus::desktop::tao::window::WindowSizeConstraints;
use dioxus::desktop::wry::dpi::{
    PhysicalSize,
    PhysicalUnit,
    PixelUnit,
    Size
};
use dioxus::desktop::{Config, WindowBuilder};

use dioxus::logger::tracing::Level;

mod constants;
mod utils;
mod stores;
mod router;
mod components;
mod hooks;
mod models;
mod views;
mod app;
use app::App;

fn main() {
    // Fix: Failed to create GBM buffer of size WidthxHeight: Invalid argument
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");

    dioxus::logger::init(Level::ERROR).expect("logger failed to init");
    dioxus::LaunchBuilder::desktop()
    .with_cfg(
        Config::new()
            .with_disable_context_menu(IS_WINDOW_CONTEXT_MENU_DISABLED)
            .with_window(
                WindowBuilder::new()
                .with_resizable(true)
                .with_decorations(false)
                // .with_focused(true)
                .with_inner_size(Size::Physical(PhysicalSize { height: 1440, width: 2560 }))
                .with_inner_size_constraints(
                    WindowSizeConstraints::new(
                        Option::Some(PixelUnit::Physical(PhysicalUnit::new(1280))),
                        Option::Some(PixelUnit::Physical(PhysicalUnit::new(720))),
                        Option::Some(PixelUnit::Physical(PhysicalUnit::new(2560))),
                        Option::Some(PixelUnit::Physical(PhysicalUnit::new(1600)))
                    )
                )
                .with_theme(Option::None)
                .with_title("Settings")
                .with_transparent(true)
                // // .with_window_icon(Option::Some(Icon::from_rgba(std::fs::read(Path::new("/assets/favicon.ico")).unwrap(), 16, 16).unwrap()))
                // .with_cursor_moved_event(false)
            )
    )
    .launch(App);
}