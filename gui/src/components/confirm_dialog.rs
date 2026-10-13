use super::{Button, Modal, Variant};
use dioxus::prelude::*;

#[component]
pub fn ConfirmDialog(
    #[props(into)] title: String,
    #[props(into)] message: String,
    #[props(into)] confirm_label: String,
    onconfirm: EventHandler<()>,
    onclose: EventHandler<()>,
) -> Element {
    rsx! {
        Modal { title, onclose,
            p { class: "text-sm text-soft", "{message}" }
            div { class: "flex justify-end gap-3",
                Button { variant: Variant::Ghost, onclick: move |_| onclose.call(()), "Cancel" }
                Button { variant: Variant::Danger, onclick: move |_| onconfirm.call(()), "{confirm_label}" }
            }
        }
    }
}
