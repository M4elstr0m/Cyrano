use super::Badge;
use dioxus::prelude::*;

#[component]
pub fn Section(
    #[props(into)] title: String,
    #[props(default)] count: Option<usize>,
    #[props(default)] open: bool,
    children: Element,
) -> Element {
    let mut expanded = use_signal(|| open);
    let indicator = if expanded() { "−" } else { "+" };

    rsx! {
        div { class: "overflow-hidden rounded-2xl border border-line bg-surface",
            button {
                class: "flex w-full items-center justify-between px-4 py-3 text-left hover:bg-inset",
                r#type: "button",
                onclick: move |_| expanded.set(!expanded()),
                span { class: "flex items-center gap-2 text-sm font-semibold",
                    "{title}"
                    if let Some(count) = count {
                        Badge { "{count}" }
                    }
                }
                span { class: "text-soft", "{indicator}" }
            }
            if expanded() {
                div { class: "flex flex-col gap-4 border-t border-line p-4", {children} }
            }
        }
    }
}
