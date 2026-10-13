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
        div { class: "relative flex h-4 w-6 items-center justify-center",
            span { class: "block h-3.5 w-6 rounded-full border-2 border-current" }
            span { class: "absolute block size-1.5 rounded-full bg-current" }
            if crossed {
                span {
                    class: "absolute block h-0.5 w-7 rounded-full bg-current",
                    style: "transform: rotate(-35deg)",
                }
            }
        }
    }
}
