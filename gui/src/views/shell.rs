use crate::{
    components::{Header, Toasts},
    route::Route,
};
use dioxus::prelude::*;

#[component]
pub fn Shell() -> Element {
    rsx! {
        div { class: "flex h-full flex-col",
            Header {}
            main { class: "min-h-0 flex-1", Outlet::<Route> {} }
            Toasts {}
        }
    }
}
