use crate::{
    components::{Button, Card, Variant},
    state::use_app_state,
};
use dioxus::prelude::*;

#[component]
pub fn EmptyTemplates() -> Element {
    let state = use_app_state();
    let directory = state.preferences.read().templates_dir().ok();
    let location = directory
        .as_ref()
        .map(|path| path.display().to_string())
        .unwrap_or_default();

    rsx! {
        Card { class: "flex max-w-xl flex-col gap-4 p-8",
            h2 { class: "text-lg font-semibold", "No templates yet" }
            p { class: "text-sm text-soft",
                "Drop a template folder containing a TEMPLATE.cyrano file into the templates directory, then rescan."
            }
            p { class: "break-all rounded-xl bg-inset px-3.5 py-2.5 font-mono text-xs text-soft",
                "{location}"
            }
            div { class: "flex gap-3",
                Button {
                    onclick: move |_| {
                        if let Some(path) = &directory {
                            let _ = opener::open(path);
                        }
                    },
                    "Open folder"
                }
                Button {
                    variant: Variant::Secondary,
                    onclick: move |_| state.reload_templates(),
                    "Rescan"
                }
            }
        }
    }
}
