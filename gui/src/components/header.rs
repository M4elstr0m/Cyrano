use super::{Button, Size, ThemeToggle, Variant};
use crate::{route::Route, state::use_app_state};
use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    let navigator = use_navigator();
    let state = use_app_state();
    let mut settings_open = state.settings_open;

    rsx! {
        header { class: "flex h-16 shrink-0 items-center justify-between border-b border-line bg-surface px-6",
            button {
                class: "flex items-center gap-3",
                r#type: "button",
                onclick: move |_| {
                    navigator.push(Route::Home {});
                },
                span { class: "flex size-9 items-center justify-center rounded-xl bg-primary text-sm font-bold text-on-primary",
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
