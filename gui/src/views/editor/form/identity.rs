use super::{super::session::use_session, support::optional};
use crate::components::{Section, TextArea, TextInput};
use dioxus::prelude::*;

#[component]
pub fn IdentitySection() -> Element {
    let mut draft = use_session().draft;
    let cv = draft.read();

    rsx! {
        Section { title: "Identity", open: true,
            TextInput {
                label: "Full name",
                value: cv.name.clone(),
                oninput: move |value| draft.write().name = value,
            }
            TextInput {
                label: "Title",
                value: cv.title.clone().unwrap_or_default(),
                oninput: move |value| draft.write().title = optional(value),
            }
            TextArea {
                label: "Description",
                value: cv.description.clone().unwrap_or_default(),
                oninput: move |value| draft.write().description = optional(value),
            }
            TextInput {
                label: "Age",
                value: cv.age_display.clone().unwrap_or_default(),
                oninput: move |value| draft.write().age_display = optional(value),
            }
            TextInput {
                label: "Location",
                value: cv.location.clone().unwrap_or_default(),
                oninput: move |value| draft.write().location = optional(value),
            }
            TextInput {
                label: "Email",
                value: cv.email.clone().unwrap_or_default(),
                oninput: move |value| draft.write().email = optional(value),
            }
        }
    }
}
