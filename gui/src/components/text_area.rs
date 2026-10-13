use super::text_input::FIELD_CLASS;
use dioxus::prelude::*;

#[component]
pub fn TextArea(
    #[props(into)] label: String,
    value: String,
    #[props(default)] placeholder: String,
    oninput: EventHandler<String>,
) -> Element {
    rsx! {
        label { class: "flex flex-col gap-1.5",
            span { class: "text-xs font-medium text-soft", "{label}" }
            textarea {
                class: "{FIELD_CLASS} min-h-24",
                rows: 4,
                value,
                placeholder,
                oninput: move |event| oninput.call(event.value()),
            }
        }
    }
}
