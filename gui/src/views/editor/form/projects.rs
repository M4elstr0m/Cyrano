use super::{
    super::session::use_session,
    date_range::date_fields,
    row::{AddButton, RowCard},
    string_list::StringListEditor,
    support::{move_item, new_range, with_item},
};
use crate::components::{Section, TextArea, TextInput};
use cyrano_core::cv_data::Project;
use dioxus::prelude::*;

#[component]
pub fn ProjectsSection() -> Element {
    let mut draft = use_session().draft;
    let count = draft.read().projects.len();

    rsx! {
        Section { title: "Projects", count,
            for index in 0..count {
                ProjectRow { key: "{index}", index }
            }
            AddButton {
                label: "Add project",
                onclick: move |_| {
                    draft
                        .write()
                        .projects
                        .push(Project {
                            title: String::new(),
                            date: new_range(),
                            description: String::new(),
                            used_skills: Vec::new(),
                        });
                },
            }
        }
    }
}

#[component]
fn ProjectRow(index: usize) -> Element {
    let mut draft = use_session().draft;
    let Some(item) = draft.read().projects.get(index).cloned() else {
        return rsx! {};
    };

    rsx! {
        RowCard {
            index,
            count: draft.read().projects.len(),
            onmove: move |delta| move_item(&mut draft.write().projects, index, delta),
            onremove: move |_| {
                draft.write().projects.remove(index);
            },
            TextInput {
                label: "Title",
                value: item.title,
                oninput: move |value| {
                    with_item(&mut draft.write().projects, index, |item| item.title = value)
                },
            }
            TextArea {
                label: "Description",
                value: item.description,
                oninput: move |value| {
                    with_item(&mut draft.write().projects, index, |item| item.description = value)
                },
            }
            StringListEditor {
                label: "Skill used",
                add_label: "Add skill",
                items: item.used_skills,
                onchange: move |skills| {
                    with_item(&mut draft.write().projects, index, |item| item.used_skills = skills)
                },
            }
            {
                date_fields(
                    &item.date,
                    move |date| {
                        with_item(&mut draft.write().projects, index, |item| item.date = date)
                    },
                )
            }
        }
    }
}
