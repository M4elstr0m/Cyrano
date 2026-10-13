use crate::components::{Button, Size, Variant};
use dioxus::prelude::*;

#[component]
pub fn RowCard(onremove: EventHandler<()>, children: Element) -> Element {
    rsx! {
        div { class: "flex flex-col gap-3 rounded-2xl border border-line p-4",
            {children}
            div { class: "flex justify-end",
                Button {
                    variant: Variant::Ghost,
                    size: Size::Sm,
                    onclick: move |_| onremove.call(()),
                    "Remove"
                }
            }
        }
    }
}

#[component]
pub fn AddButton(#[props(into)] label: String, onclick: EventHandler<()>) -> Element {
    rsx! {
        Button {
            variant: Variant::Secondary,
            size: Size::Sm,
            class: "self-start",
            onclick: move |_| onclick.call(()),
            "+ {label}"
        }
    }
}
