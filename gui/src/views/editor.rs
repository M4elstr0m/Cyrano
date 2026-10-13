use crate::{
    components::{Button, Card},
    route::Route,
    state::use_app_state,
};
use dioxus::prelude::*;
use std::path::PathBuf;

mod action_bar;
mod assets_section;
mod create_profile_dialog;
mod form;
mod preview_pane;
mod profile_picker;
mod session;
mod settings_pane;
mod unlock_dialog;

use preview_pane::PreviewPane;
use session::{Session, use_autosave};
use settings_pane::SettingsPane;

#[component]
pub fn Editor(template: String) -> Element {
    let state = use_app_state();

    let dir = state
        .template_dir(&template)
        .filter(|dir| state.templates.read().template(dir).is_some());

    match dir {
        Some(dir) => rsx! {
            Workspace { key: "{template}", dir }
        },
        None => rsx! {
            MissingTemplate {}
        },
    }
}

#[component]
fn Workspace(dir: PathBuf) -> Element {
    let state = use_app_state();
    let session = use_context_provider(|| Session::new(state, dir));
    use_autosave(session);

    rsx! {
        div { class: "flex h-full",
            PreviewPane {}
            SettingsPane {}
        }
    }
}

#[component]
fn MissingTemplate() -> Element {
    let navigator = use_navigator();

    rsx! {
        div { class: "flex h-full items-center justify-center p-8",
            Card { class: "flex max-w-md flex-col gap-4 p-8",
                h2 { class: "text-lg font-semibold", "Template not found" }
                p { class: "text-sm text-soft", "This template is no longer in your templates folder." }
                Button {
                    onclick: move |_| {
                        navigator.push(Route::Home {});
                    },
                    "Back to templates"
                }
            }
        }
    }
}
