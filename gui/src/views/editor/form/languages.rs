use super::{
    super::session::use_session,
    row::{AddButton, RowCard},
    support::{move_item, with_item},
};
use crate::components::{Section, TextInput};
use cyrano_core::cv_data::LanguageSkill;
use dioxus::prelude::*;

#[component]
pub fn LanguagesSection() -> Element {
    let mut draft = use_session().draft;
    let count = draft.read().languages.len();

    rsx! {
        Section { title: "Languages", count,
            for index in 0..count {
                LanguageRow { key: "{index}", index }
            }
            AddButton {
                label: "Add language",
                onclick: move |_| {
                    draft
                        .write()
                        .languages
                        .push(LanguageSkill {
                            name: String::new(),
                            level: String::new(),
                        });
                },
            }
        }
    }
}

#[component]
fn LanguageRow(index: usize) -> Element {
    let mut draft = use_session().draft;
    let Some(language) = draft.read().languages.get(index).cloned() else {
        return rsx! {};
    };

    rsx! {
        RowCard {
            index,
            count: draft.read().languages.len(),
            onmove: move |delta| move_item(&mut draft.write().languages, index, delta),
            onremove: move |_| {
                draft.write().languages.remove(index);
            },
            TextInput {
                label: "Language",
                value: language.name,
                oninput: move |value| {
                    with_item(&mut draft.write().languages, index, |language| language.name = value)
                },
            }
            TextInput {
                label: "Level",
                value: language.level,
                oninput: move |value| {
                    with_item(&mut draft.write().languages, index, |language| language.level = value)
                },
            }
        }
    }
}
