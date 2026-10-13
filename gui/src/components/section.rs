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

    rsx! {
        div { class: "flex flex-col rounded-2xl border border-line bg-surface",
            button {
                class: "flex w-full items-center justify-between fade-hover rounded-2xl px-4 py-3 text-left",
                r#type: "button",
                onclick: move |_| expanded.set(!expanded()),
                span { class: "flex items-center gap-2 text-sm font-semibold",
                    "{title}"
                    if let Some(count) = count {
                        Badge { "{count}" }
                    }
                }
                span {
                    class: "turn mb-1 block size-2 border-b-2 border-r-2 border-current text-soft",
                    style: if expanded() { "transform: rotate(225deg)" } else { "transform: rotate(45deg)" },
                }
            }
            if expanded() {
                div { class: "animate-fade-up flex flex-col gap-4 border-t border-line p-4",
                    {children}
                }
            }
        }
    }
}
