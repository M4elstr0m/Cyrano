use dioxus::prelude::*;

mod session;

#[component]
pub fn Editor(template: String) -> Element {
    rsx! {
        div { class: "p-8", "{template}" }
    }
}
