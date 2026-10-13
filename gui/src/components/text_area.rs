use dioxus::prelude::*;

#[component]
pub fn TextArea(
    #[props(into)] label: String,
    value: String,
    #[props(default)] placeholder: String,
    oninput: EventHandler<String>,
) -> Element {
    let show_placeholder = value.is_empty() && !placeholder.is_empty();

    rsx! {
        div { class: "flex flex-col gap-1.5",
            span { class: "text-xs font-medium text-soft", "{label}" }
            div { class: "ring-within relative flex rounded-xl border border-line bg-inset",
                if show_placeholder {
                    span { class: "pointer-events-none absolute -z-10 left-0 top-0 px-3.5 py-2.5 text-sm text-soft opacity-70",
                        "{placeholder}"
                    }
                }
                textarea {
                    class: "relative z-10 min-h-24 w-full bg-transparent px-3.5 py-2.5 text-sm text-ink outline-none",
                    rows: 4,
                    value,
                    oninput: move |event| oninput.call(event.value()),
                }
            }
        }
    }
}
