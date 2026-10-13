use crate::{preferences::Theme, state::use_app_state};
use dioxus::prelude::*;

#[component]
pub fn ThemeToggle() -> Element {
    let state = use_app_state();
    let current = state.theme();

    rsx! {
        div { class: "flex rounded-full border border-line bg-inset p-1",
            ThemeOption {
                label: "Light",
                active: current == Theme::Light,
                onclick: move |_| state.set_theme(Theme::Light),
            }
            ThemeOption {
                label: "Dark",
                active: current == Theme::Dark,
                onclick: move |_| state.set_theme(Theme::Dark),
            }
        }
    }
}

#[component]
fn ThemeOption(label: String, active: bool, onclick: EventHandler<MouseEvent>) -> Element {
    let state_class = if active {
        "bg-primary text-on-primary"
    } else {
        "fade-hover text-soft hover:text-ink"
    };

    rsx! {
        button {
            class: "rounded-full px-3.5 py-1 text-xs font-medium {state_class}",
            r#type: "button",
            onclick: move |event| onclick.call(event),
            "{label}"
        }
    }
}
