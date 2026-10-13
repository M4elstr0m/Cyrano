use super::{
    super::session::use_session,
    date_range::date_fields,
    row::{AddButton, RowCard},
    support::{move_to, new_range, with_item},
};
use crate::components::{Section, TextInput};
use cyrano_core::cv_data::Experience;
use dioxus::prelude::*;

#[component]
pub fn ExperienceSection() -> Element {
    let mut draft = use_session().draft;
    let count = draft.read().experience.len();

    rsx! {
        Section { title: "Experience", count,
            for index in 0..count {
                ExperienceRow { key: "{index}", index }
            }
            AddButton {
                label: "Add experience",
                onclick: move |_| {
                    draft
                        .write()
                        .experience
                        .push(Experience {
                            title: String::new(),
                            company: String::new(),
                            date: new_range(),
                        });
                },
            }
        }
    }
}

#[component]
fn ExperienceRow(index: usize) -> Element {
    let mut draft = use_session().draft;
    let Some(item) = draft.read().experience.get(index).cloned() else {
        return rsx! {};
    };

    rsx! {
        RowCard {
            list: "experience",
            index,
            onreorder: move |(from, to)| move_to(&mut draft.write().experience, from, to),
            onremove: move |_| {
                draft.write().experience.remove(index);
            },
            TextInput {
                label: "Role",
                placeholder: "e.g. Backend Engineer",
                value: item.title,
                oninput: move |value| {
                    with_item(&mut draft.write().experience, index, |item| item.title = value)
                },
            }
            TextInput {
                label: "Company",
                placeholder: "e.g. Acme",
                value: item.company,
                oninput: move |value| {
                    with_item(&mut draft.write().experience, index, |item| item.company = value)
                },
            }
            {
                date_fields(
                    &item.date,
                    move |date| {
                        with_item(&mut draft.write().experience, index, |item| item.date = date)
                    },
                )
            }
        }
    }
}
