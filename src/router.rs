use dioxus::prelude::*;

use crate::components::MainLayout;
use crate::views::{Home, CategorizedSettings};


#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(MainLayout)]
    #[route("/")]
    Home {},
    #[route("/category/:category_name")]
    CategorizedSettings { category_name: String },
}