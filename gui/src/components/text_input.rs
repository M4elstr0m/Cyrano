use dioxus::prelude::*;

#[component]
pub fn TextInput(
    #[props(into)] label: String,
    value: String,
    #[props(default)] placeholder: String,
    #[props(default)] autofocus: bool,
    oninput: EventHandler<String>,
    #[props(default)] onenter: EventHandler<()>,
) -> Element {
    let show_placeholder = value.is_empty() && !placeholder.is_empty();

    rsx! {
        div { class: "flex flex-col gap-1.5",
            span { class: "text-xs font-medium text-soft", "{label}" }
            div { class: "ring-within relative flex items-center rounded-xl border border-line bg-inset",
                if show_placeholder {
                    span { class: "pointer-events-none absolute inset-y-0 left-0 flex items-center pl-3.5 text-sm text-soft opacity-70",
                        "{placeholder}"
                    }
                }
                input {
                    class: "relative z-10 w-full bg-transparent px-3.5 py-2.5 text-sm text-ink outline-none",
                    r#type: "text",
                    value,
                    autofocus,
                    oninput: move |event| oninput.call(event.value()),
                    onkeydown: move |event| {
                        if event.key() == Key::Enter {
                            onenter.call(());
                        }
                    },
                }
            }
        }
    }
}
