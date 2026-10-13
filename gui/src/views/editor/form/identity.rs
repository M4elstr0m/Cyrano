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
                placeholder: "e.g. Cyrano de Bergerac",
                value: cv.name.clone(),
                oninput: move |value| draft.write().name = value,
            }
            TextInput {
                label: "Title",
                placeholder: "e.g. Software Engineer",
                value: cv.title.clone().unwrap_or_default(),
                oninput: move |value| draft.write().title = optional(value),
            }
            TextArea {
                label: "Description",
                placeholder: "A short summary of who you are",
                value: cv.description.clone().unwrap_or_default(),
                oninput: move |value| draft.write().description = optional(value),
            }
            TextInput {
                label: "Age",
                placeholder: "e.g. 28 years old",
                value: cv.age_display.clone().unwrap_or_default(),
                oninput: move |value| draft.write().age_display = optional(value),
            }
            TextInput {
                label: "Location",
                placeholder: "e.g. Lyon, France",
                value: cv.location.clone().unwrap_or_default(),
                oninput: move |value| draft.write().location = optional(value),
            }
            TextInput {
                label: "Email",
                placeholder: "name@example.com",
                value: cv.email.clone().unwrap_or_default(),
                oninput: move |value| draft.write().email = optional(value),
            }
            TextInput {
                label: "Phone number",
                placeholder: "e.g. +33 6 12 34 56 78",
                value: cv.phonenumber.clone().unwrap_or_default(),
                oninput: move |value| draft.write().phonenumber = optional(value),
            }
        }
    }
}
