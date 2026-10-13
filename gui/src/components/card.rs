use dioxus::prelude::*;

#[component]
pub fn Card(#[props(default)] class: String, children: Element) -> Element {
    rsx! {
        div { class: "rounded-3xl border border-line bg-surface shadow-card {class}",
            {children}
        }
    }
}
