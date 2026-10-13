use super::{Editor, Home, settings_dialog::SettingsDialog};
use crate::{
    components::{Header, Toasts},
    state::{Screen, use_app_state},
};
use dioxus::prelude::*;

#[component]
pub fn Shell() -> Element {
    let state = use_app_state();
    let screen = (state.screen)();
    let opened = state.opened.read().clone();

    rsx! {
        div { class: "flex h-full flex-col",
            Header {}
            main { class: "flex min-h-0 flex-1",
                div { class: if screen == Screen::Home { "flex h-full w-full" } else { "hidden" },
                    Home {}
                }
                for folder in opened {
                    div {
                        key: "{folder}",
                        class: if screen == Screen::Editor(folder.clone()) { "flex h-full w-full" } else { "hidden" },
                        Editor { template: folder.clone() }
                    }
                }
            }
            if (state.settings_open)() {
                SettingsDialog {}
            }
            Toasts {}
        }
    }
}
