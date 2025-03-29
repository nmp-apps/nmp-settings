use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct IconProps {
    // currentColor means take color from parent element
    #[props(into, default = "currentColor")]
    pub color: Option<String>,
    #[props(into, default = "24px")]
    pub size: Option<String>
}

mod close;
pub use close::*;

mod extension;
pub use extension::*;

mod minimize;
pub use minimize::*;

mod square;
pub use square::*;
