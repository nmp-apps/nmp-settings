use dioxus::prelude::*;

#[derive(PartialEq, Props, Clone)]
pub struct CategorizedSettingsProps {
    category_name: String,
}

#[component]
pub fn CategorizedSettings(props: CategorizedSettingsProps) -> Element {
    rsx! {
        {format!("CategorizedSettings: {}", props.category_name)}
    }
}
