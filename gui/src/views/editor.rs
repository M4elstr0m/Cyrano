use crate::state::use_app_state;
use dioxus::prelude::*;
use std::path::PathBuf;

mod action_bar;
mod assets_section;
mod create_profile_dialog;
mod dialogs;
mod form;
mod preview_pane;
mod profile_picker;
mod session;
mod settings_pane;
mod unlock_dialog;

use dialogs::Dialogs;
use preview_pane::PreviewPane;
use session::{Session, use_autosave};
use settings_pane::SettingsPane;

#[component]
pub fn Editor(template: String) -> Element {
    let state = use_app_state();
    let dir = state.template_dir(&template).unwrap_or_default();

    rsx! {
        Workspace { dir }
    }
}

#[component]
fn Workspace(dir: PathBuf) -> Element {
    let state = use_app_state();
    let session = use_context_provider(|| Session::new(state, dir));
    use_autosave(session);
    use_effect(move || {
        if session.is_current() {
            spawn(async move { session.sync() });
        }
    });
    use_hook(move || state.on_quit(Callback::new(move |_| session.persist())));

    let mut drag = session.drag;

    rsx! {
        div {
            class: "flex h-full w-full",
            onmouseup: move |_| drag.set(None),
            PreviewPane {}
            SettingsPane {}
            Dialogs {}
        }
    }
}
