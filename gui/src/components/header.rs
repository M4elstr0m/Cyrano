use super::{Button, Size, ThemeToggle, Variant};
use crate::state::use_app_state;
use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    let state = use_app_state();
    let mut settings_open = state.settings_open;

    rsx! {
        header { class: "flex h-16 shrink-0 items-center justify-between border-b border-line bg-surface px-6",
            button {
                class: "flex items-center gap-3",
                r#type: "button",
                onclick: move |_| state.go_home(),
                span { class: "flex size-9 items-center justify-center rounded-xl bg-brand text-sm font-bold text-white shadow-card",
                    "C"
                }
                span { class: "text-base font-semibold tracking-tight", "Cyrano" }
            }
            div { class: "flex items-center gap-3",
                ThemeToggle {}
                Button {
                    variant: Variant::Secondary,
                    size: Size::Sm,
                    onclick: move |_| settings_open.set(true),
                    "Settings"
                }
            }
        }
    }
}
