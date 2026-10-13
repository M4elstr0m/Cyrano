use super::{Button, Size, Variant};
use crate::state::{Notice, use_app_state};
use dioxus::prelude::*;

#[component]
pub fn Toasts() -> Element {
    let state = use_app_state();
    let notices = state.notices.read();

    rsx! {
        div { class: "fixed bottom-5 right-5 z-[60] flex w-96 flex-col gap-3",
            for (index, notice) in notices.iter().enumerate() {
                Toast {
                    key: "{index}-{notice.message}",
                    notice: notice.clone(),
                    index,
                }
            }
        }
    }
}

#[component]
fn Toast(notice: Notice, index: usize) -> Element {
    let state = use_app_state();
    let tone_class = notice.tone.text_class();

    rsx! {
        div { class: "animate-slide-in flex flex-col gap-3 rounded-2xl border border-line bg-surface p-4 shadow-overlay",
            p { class: "break-words text-sm {tone_class}", "{notice.message}" }
            div { class: "flex justify-end gap-2",
                if let Some(path) = notice.reveal.clone() {
                    Button {
                        variant: Variant::Secondary,
                        size: Size::Sm,
                        onclick: move |_| reveal(&path),
                        "Show folder"
                    }
                }
                Button {
                    variant: Variant::Ghost,
                    size: Size::Sm,
                    onclick: move |_| state.dismiss(index),
                    "Dismiss"
                }
            }
        }
    }
}

fn reveal(path: &std::path::Path) {
    let _ = opener::open(path.parent().unwrap_or(path));
}
