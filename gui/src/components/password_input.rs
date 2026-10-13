use dioxus::prelude::*;

#[component]
pub fn PasswordInput(
    #[props(into)] label: String,
    value: String,
    #[props(default)] autofocus: bool,
    oninput: EventHandler<String>,
    #[props(default)] onenter: EventHandler<()>,
) -> Element {
    let mut revealed = use_signal(|| false);
    let dots = "•".repeat(value.chars().count());
    let text_class = if revealed() {
        "text-ink"
    } else {
        "text-transparent"
    };

    rsx! {
        div { class: "flex flex-col gap-1.5",
            span { class: "text-xs font-medium text-soft", "{label}" }
            div { class: "ring-within relative flex items-center rounded-xl border border-line bg-inset",
                if !revealed() {
                    span { class: "absolute inset-y-0 left-0 flex items-center pl-3.5 text-sm tracking-widest text-ink",
                        "{dots}"
                    }
                }
                input {
                    class: "w-full bg-transparent px-3.5 py-2.5 text-sm outline-none {text_class}",
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
                button {
                    class: "press px-3 text-soft hover:text-accent",
                    r#type: "button",
                    onclick: move |_| revealed.set(!revealed()),
                    Eye { crossed: !revealed() }
                }
            }
        }
    }
}

#[component]
fn Eye(crossed: bool) -> Element {
    rsx! {
        svg {
            width: "24",
            height: "16",
            view_box: "0 0 24 16",
            fill: "none",
            path {
                d: "M1.5 8 C4.5 3 8 1.5 12 1.5 C16 1.5 19.5 3 22.5 8 C19.5 13 16 14.5 12 14.5 C8 14.5 4.5 13 1.5 8 Z",
                stroke: "currentColor",
                stroke_width: "1.8",
                stroke_linejoin: "round",
            }
            circle {
                cx: "12",
                cy: "8",
                r: "2.6",
                fill: "currentColor",
            }
            if crossed {
                path {
                    d: "M3.5 14.5 L20.5 1.5",
                    stroke: "currentColor",
                    stroke_width: "1.8",
                    stroke_linecap: "round",
                }
            }
        }
    }
}
