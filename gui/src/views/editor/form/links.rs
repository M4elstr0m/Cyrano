use super::{
    super::session::use_session,
    row::{AddButton, RowCard},
    support::with_item,
};
use crate::components::{Section, TextInput};
use cyrano_core::cv_data::Link;
use dioxus::prelude::*;

#[component]
pub fn LinksSection() -> Element {
    let mut draft = use_session().draft;
    let count = draft.read().links.len();

    rsx! {
        Section { title: "Links", count,
            for index in 0..count {
                LinkRow { key: "{index}", index }
            }
            AddButton {
                label: "Add link",
                onclick: move |_| {
                    draft
                        .write()
                        .links
                        .push(Link {
                            label: String::new(),
                            url: String::new(),
                        });
                },
            }
        }
    }
}

#[component]
fn LinkRow(index: usize) -> Element {
    let mut draft = use_session().draft;
    let Some(link) = draft.read().links.get(index).cloned() else {
        return rsx! {};
    };

    rsx! {
        RowCard {
            onremove: move |_| {
                draft.write().links.remove(index);
            },
            TextInput {
                label: "Label",
                value: link.label,
                oninput: move |value| {
                    with_item(&mut draft.write().links, index, |link| link.label = value)
                },
            }
            TextInput {
                label: "URL",
                value: link.url,
                oninput: move |value| { with_item(&mut draft.write().links, index, |link| link.url = value) },
            }
        }
    }
}
