use crate::{
    components::{Button, Modal, Size, Variant},
    state::use_app_state,
};
use dioxus::prelude::*;
use std::path::PathBuf;

#[component]
pub fn SettingsDialog() -> Element {
    let state = use_app_state();
    let mut open = state.settings_open;
    let preferences = state.preferences.read();

    let locations = [
        ("Templates", preferences.templates_dir()),
        ("Profiles", preferences.profiles_dir()),
        ("Output", preferences.output_dir()),
    ];

    rsx! {
        Modal { title: "Settings", onclose: move |_| open.set(false),
            div { class: "flex flex-col gap-4",
                for (label , path) in locations {
                    if let Ok(path) = path {
                        LocationRow { key: "{label}", label, path }
                    }
                }
            }
            div { class: "flex flex-col gap-3 border-t border-line pt-4",
                div { class: "flex flex-col gap-1",
                    span { class: "text-sm font-semibold", "Maintenance" }
                    p { class: "text-xs text-soft",
                        "Remove the leftover files created while rendering previews."
                    }
                }
                Button {
                    variant: Variant::Secondary,
                    class: "self-start",
                    onclick: move |_| state.clear_temp_files(),
                    "Clear temporary files"
                }
            }
        }
    }
}

#[component]
fn LocationRow(label: &'static str, path: PathBuf) -> Element {
    let text = path.display().to_string();

    rsx! {
        div { class: "flex flex-col gap-1.5",
            span { class: "text-xs font-medium text-soft", "{label}" }
            div { class: "flex items-center gap-2",
                p { class: "min-w-0 flex-1 break-all rounded-xl bg-inset px-3.5 py-2.5 font-mono text-xs text-soft",
                    "{text}"
                }
                Button {
                    variant: Variant::Secondary,
                    size: Size::Sm,
                    onclick: move |_| {
                        let _ = opener::open(&path);
                    },
                    "Open"
                }
            }
        }
    }
}
