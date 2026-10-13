use super::session::{Preview, use_session};
use crate::components::{Button, Variant};
use dioxus::prelude::*;

#[component]
pub fn ActionBar() -> Element {
    let session = use_session();
    let blocker = session.blocker();
    let disabled = blocker.is_some() || session.busy();
    let previewing = *session.preview.read() == Preview::Rendering;
    let exporting = *session.exporting.read();

    rsx! {
        div { class: "flex flex-col gap-3 border-t border-line p-4",
            if let Some(hint) = blocker {
                p { class: "text-xs text-soft", "{hint}" }
            }
            div { class: "flex gap-3",
                Button {
                    variant: Variant::Secondary,
                    class: "flex-1",
                    disabled,
                    onclick: move |_| session.run_preview(),
                    if previewing {
                        "Rendering…"
                    } else {
                        "Preview"
                    }
                }
                Button {
                    class: "flex-1",
                    disabled,
                    onclick: move |_| session.run_export(),
                    if exporting {
                        "Creating…"
                    } else {
                        "Create CV"
                    }
                }
            }
        }
    }
}
