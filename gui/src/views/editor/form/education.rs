use super::{
    super::session::use_session,
    date_range::date_fields,
    row::{AddButton, RowCard},
    support::{new_range, with_item},
};
use crate::components::{Section, TextArea, TextInput};
use cyrano_core::cv_data::Education;
use dioxus::prelude::*;

#[component]
pub fn EducationSection() -> Element {
    let mut draft = use_session().draft;
    let count = draft.read().education.len();

    rsx! {
        Section { title: "Education", count,
            for index in 0..count {
                EducationRow { key: "{index}", index }
            }
            AddButton {
                label: "Add education",
                onclick: move |_| {
                    draft
                        .write()
                        .education
                        .push(Education {
                            title: String::new(),
                            date: new_range(),
                            description: String::new(),
                        });
                },
            }
        }
    }
}

#[component]
fn EducationRow(index: usize) -> Element {
    let mut draft = use_session().draft;
    let Some(item) = draft.read().education.get(index).cloned() else {
        return rsx! {};
    };

    rsx! {
        RowCard {
            onremove: move |_| {
                draft.write().education.remove(index);
            },
            TextInput {
                label: "Title",
                value: item.title,
                oninput: move |value| {
                    with_item(&mut draft.write().education, index, |item| item.title = value)
                },
            }
            TextArea {
                label: "Description",
                value: item.description,
                oninput: move |value| {
                    with_item(&mut draft.write().education, index, |item| item.description = value)
                },
            }
            {
                date_fields(
                    &item.date,
                    move |date| {
                        with_item(&mut draft.write().education, index, |item| item.date = date)
                    },
                )
            }
        }
    }
}
