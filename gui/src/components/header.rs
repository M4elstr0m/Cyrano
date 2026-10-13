use super::{Button, DragRegion, Logo, Size, ThemeToggle, Variant, WindowControls};
use crate::state::use_app_state;
use dioxus::prelude::*;

#[component]
pub fn Header() -> Element {
    let state = use_app_state();
    let mut settings_open = state.settings_open;

    rsx! {
        header { class: "flex h-16 shrink-0 select-none items-center gap-4 border-b border-line bg-surface pl-6 pr-3",
            button {
                class: "flex items-center gap-3",
                r#type: "button",
                onclick: move |_| state.go_home(),
                Logo { class: "size-10" }
                span { class: "text-lg font-bold tracking-tight", "Cyrano" }
            }
            DragRegion {}
            div { class: "flex items-center gap-3",
                ThemeToggle {}
                Button {
                    variant: Variant::Secondary,
                    size: Size::Sm,
                    onclick: move |_| settings_open.set(true),
                    "Settings"
                }
                WindowControls {}
            }
        }
    }
}
