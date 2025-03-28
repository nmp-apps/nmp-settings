use dioxus::prelude::*;

use crate::components::MainLayout;
use crate::views::Home;


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(MainLayout)]
    #[route("/")]
    Home {},
}