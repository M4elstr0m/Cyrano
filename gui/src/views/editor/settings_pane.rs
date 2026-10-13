use super::{
    action_bar::ActionBar, assets_section::AssetsSection, form::CvForm,
    profile_picker::ProfilePicker, session::use_session,
};
use crate::{
    components::{Button, Size, Variant},
    route::Route,
};
use dioxus::prelude::*;

#[component]
pub fn SettingsPane() -> Element {
    let session = use_session();
    let navigator = use_navigator();
    let name = session.template_name();

    rsx! {
        aside { class: "flex w-[440px] shrink-0 flex-col border-l border-line bg-surface",
            div { class: "flex items-center justify-between gap-3 border-b border-line px-6 py-4",
                div { class: "flex min-w-0 flex-col",
                    span { class: "text-xs font-medium text-soft", "Template" }
                    h1 { class: "truncate text-base font-semibold", "{name}" }
                }
                Button {
                    variant: Variant::Secondary,
                    size: Size::Sm,
                    onclick: move |_| {
                        session.persist();
                        navigator.push(Route::Home {});
                    },
                    "Change"
                }
            }
            div { class: "flex min-h-0 flex-1 flex-col gap-5 overflow-y-auto p-6",
                ProfilePicker {}
                AssetsSection {}
                if session.active.read().is_some() {
                    CvForm {}
                }
            }
            ActionBar {}
        }
    }
}
