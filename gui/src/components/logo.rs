use dioxus::prelude::*;

const LOGO_SMALL: Asset = asset!("/assets/logo-80.png");
const LOGO_LARGE: Asset = asset!("/assets/logo-192.png");

#[component]
pub fn Logo(#[props(default)] class: String, #[props(default)] large: bool) -> Element {
    let src = if large { LOGO_LARGE } else { LOGO_SMALL };

    rsx! {
        img {
            class: "shrink-0 rounded-full border border-line object-cover shadow-card {class}",
            src,
            alt: "Cyrano",
        }
    }
}
