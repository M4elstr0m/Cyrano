use super::{super::session::use_session, string_list::StringListEditor};
use crate::components::Section;
use dioxus::prelude::*;

#[component]
pub fn SoftSkillsSection() -> Element {
    let mut draft = use_session().draft;
    let items = draft.read().soft_skills.clone();

    rsx! {
        Section { title: "Soft skills", count: items.len(),
            StringListEditor {
                label: "Skill",
                add_label: "Add skill",
                items,
                onchange: move |items| draft.write().soft_skills = items,
            }
        }
    }
}

#[component]
pub fn HobbiesSection() -> Element {
    let mut draft = use_session().draft;
    let items = draft.read().hobbies.clone();

    rsx! {
        Section { title: "Hobbies", count: items.len(),
            StringListEditor {
                label: "Hobby",
                add_label: "Add hobby",
                items,
                onchange: move |items| draft.write().hobbies = items,
            }
        }
    }
}
