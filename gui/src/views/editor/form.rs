use dioxus::prelude::*;
use identity::IdentitySection;
use links::LinksSection;

mod date_range;
mod identity;
mod links;
mod row;
mod string_list;
mod support;

#[component]
pub fn CvForm() -> Element {
    rsx! {
        IdentitySection {}
        LinksSection {}
    }
}
