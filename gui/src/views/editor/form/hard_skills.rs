use super::{
    super::session::use_session,
    row::{AddButton, RowCard},
    string_list::StringListEditor,
    support::{move_item, with_item},
};
use crate::components::{Section, TextInput};
use cyrano_core::cv_data::SkillGroup;
use dioxus::prelude::*;

#[component]
pub fn HardSkillsSection() -> Element {
    let mut draft = use_session().draft;
    let count = draft.read().hard_skills.len();

    rsx! {
        Section { title: "Technical skills", count,
            for index in 0..count {
                SkillGroupRow { key: "{index}", index }
            }
            AddButton {
                label: "Add skill group",
                onclick: move |_| {
                    draft
                        .write()
                        .hard_skills
                        .push(SkillGroup {
                            category: String::new(),
                            items: Vec::new(),
                        });
                },
            }
        }
    }
}

#[component]
fn SkillGroupRow(index: usize) -> Element {
    let mut draft = use_session().draft;
    let Some(group) = draft.read().hard_skills.get(index).cloned() else {
        return rsx! {};
    };

    rsx! {
        RowCard {
            index,
            count: draft.read().hard_skills.len(),
            onmove: move |delta| move_item(&mut draft.write().hard_skills, index, delta),
            onremove: move |_| {
                draft.write().hard_skills.remove(index);
            },
            TextInput {
                label: "Category",
                value: group.category,
                oninput: move |value| {
                    with_item(&mut draft.write().hard_skills, index, |group| group.category = value)
                },
            }
            StringListEditor {
                label: "Skill",
                add_label: "Add skill",
                items: group.items,
                onchange: move |items| {
                    with_item(&mut draft.write().hard_skills, index, |group| group.items = items)
                },
            }
        }
    }
}
