use super::row::AddButton;
use crate::components::{Button, Size, TextInput, Variant};
use dioxus::prelude::*;

#[component]
pub fn StringListEditor(
    #[props(into)] label: String,
    #[props(into)] add_label: String,
    items: Vec<String>,
    onchange: EventHandler<Vec<String>>,
) -> Element {
    rsx! {
        div { class: "flex flex-col gap-3",
            for (index, item) in items.iter().enumerate() {
                div { key: "{index}", class: "flex items-end gap-2",
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
                        variant: Variant::Ghost,
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
