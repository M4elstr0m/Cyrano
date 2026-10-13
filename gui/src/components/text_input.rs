use dioxus::prelude::*;

pub const FIELD_CLASS: &str = "w-full rounded-xl border border-line bg-inset px-3.5 py-2.5 text-sm text-ink outline-none ring-focus";

#[component]
pub fn TextInput(
    #[props(into)] label: String,
    value: String,
    #[props(default)] placeholder: String,
    #[props(default)] autofocus: bool,
    oninput: EventHandler<String>,
    #[props(default)] onenter: EventHandler<()>,
) -> Element {
    rsx! {
        label { class: "flex flex-col gap-1.5",
            span { class: "text-xs font-medium text-soft", "{label}" }
            input {
                class: FIELD_CLASS,
                r#type: "text",
                value,
                placeholder,
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
