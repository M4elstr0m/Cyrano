use super::settings_dialog::SettingsDialog;
use crate::{
    components::{Header, Toasts},
    route::Route,
    state::use_app_state,
};
use dioxus::prelude::*;

#[component]
pub fn Shell() -> Element {
    let state = use_app_state();

    rsx! {
        div { class: "flex h-full flex-col",
            Header {}
            main { class: "min-h-0 flex-1", Outlet::<Route> {} }
            if (state.settings_open)() {
                SettingsDialog {}
            }
            Toasts {}
        }
    }
}
