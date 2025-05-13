use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct IconProps {
    // currentColor means take color from parent element
    #[props(into, default = "currentColor")]
    pub color: Option<String>,
    #[props(into, default = "24px")]
    pub size: Option<String>,
    pub class: Option<String>,
}

mod appearance;
pub use  appearance::*;

mod bluetooth;
pub use bluetooth::*;

mod check;
pub use check::*;

mod close;
pub use close::*;

mod desktop;
pub use desktop::*;

mod display;
pub use display::*;

mod extension;
pub use extension::*;

mod info;
pub use info::*;

mod keyboard_arrow_up;
pub use keyboard_arrow_up:: *;

mod lock_screen;
pub use lock_screen::*;

mod minimize;
pub use minimize::*;

mod network;
pub use network::*;

mod notifications;
pub use notifications::*;

mod security;
pub use security::*;

mod sound;
pub use sound::*;

mod square;
pub use square::*;

mod users;
pub use users::*;
