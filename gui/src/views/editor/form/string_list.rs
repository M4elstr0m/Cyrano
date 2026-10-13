use super::super::session::{Drag, use_session};
use super::{
    row::{AddButton, Grip},
    support::move_to,
};
use crate::components::{Button, Size, TextInput, Variant};
use dioxus::prelude::*;

#[component]
pub fn StringListEditor(
    #[props(into)] list: String,
    #[props(into)] label: String,
    #[props(into)] add_label: String,
    items: Vec<String>,
    onchange: EventHandler<Vec<String>>,
) -> Element {
    let mut drag = use_session().drag;

    rsx! {
        div { class: "flex flex-col gap-3",
            for (index , item) in items.iter().enumerate() {
                div {
                    key: "{index}",
                    class: "animate-fade-up flex items-end gap-2",
                    onmousemove: {
                        let list = list.clone();
                        let items = items.clone();
                        move |_| {
                            if let Some(active) = drag()
                                && active.list == list
                                && active.index != index
                            {
                                let mut items = items.clone();
                                move_to(&mut items, active.index, index);
                                onchange.call(items);
                                drag.set(Some(Drag {
                                    list: list.clone(),
                                    index,
                                }));
                            }
                        }
                    },
                    div { class: "pb-1", Grip { list: list.clone(), index } }
                    div { class: "min-w-0 flex-1",
                        TextInput {
                            label: label.clone(),
                            value: item.clone(),
                            oninput: {
                                let items = items.clone();
                                move |value: String| {
                                    let mut items = items.clone();
                                    items[index] = value;
                                    onchange.call(items);
                                }
                            },
                        }
                    }
                    Button {
                        variant: Variant::DangerGhost,
                        size: Size::Sm,
                        onclick: {
                            let items = items.clone();
                            move |_| {
                                let mut items = items.clone();
                                items.remove(index);
                                onchange.call(items);
                            }
                        },
                        "Remove"
                    }
                }
            }
            AddButton {
                label: add_label,
                onclick: {
                    let items = items.clone();
                    move |_| {
                        let mut items = items.clone();
                        items.push(String::new());
                        onchange.call(items);
                    }
                },
            }
        }
    }
}
