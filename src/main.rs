// use std::path::Path;
use dioxus::desktop::{
    tao::window::WindowSizeConstraints,
    wry::dpi::{
        PhysicalUnit,
        PixelUnit
    },
    Config,
    WindowBuilder
};

use dioxus::logger::tracing::Level;

mod macros; // must be defined before all
mod stores;
mod router;
mod components;
mod models;
mod views;
mod app;
use app::App;

fn main() {
    dioxus::logger::init(Level::ERROR).expect("logger failed to init");
    dioxus::LaunchBuilder::desktop()
    .with_cfg(
        Config::new().with_window(
            WindowBuilder::new()
            .with_resizable(true)
            .with_decorations(false)
            // .with_focused(true)
            .with_inner_size_constraints(
                WindowSizeConstraints::new(
                    Option::Some(PixelUnit::Physical(PhysicalUnit::new(800))),
                    Option::Some(PixelUnit::Physical(PhysicalUnit::new(600))),
                    Option::None,
                    Option::None
                )
            )
            .with_theme(Option::None)
            .with_title("Settings")
            .with_transparent(true)
            // // .with_window_icon(Option::Some(Icon::from_rgba(std::fs::read(Path::new("/assets/favicon.ico")).unwrap(), 16, 16).unwrap()))
            // .with_cursor_moved_event(true)
        )
    )
    .launch(App);
}