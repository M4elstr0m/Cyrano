use dioxus::prelude::*;
use identity::IdentitySection;

mod date_range;
mod identity;
mod row;
mod string_list;
mod support;

#[component]
pub fn CvForm() -> Element {
    rsx! {
        IdentitySection {}
    }
}
