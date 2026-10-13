use super::super::session::{Drag, use_session};
use crate::components::{Button, Size, Variant};
use dioxus::prelude::*;

#[component]
pub fn RowCard(
    #[props(into)] list: String,
    index: usize,
    onreorder: EventHandler<(usize, usize)>,
    onremove: EventHandler<()>,
    children: Element,
) -> Element {
    let mut drag = use_session().drag;
    let dragging = drag().is_some_and(|active| active.list == list && active.index == index);
    let state_class = if dragging { "lifted" } else { "" };
    let hover_list = list.clone();

    rsx! {
        div {
            class: "animate-fade-up flex flex-col gap-3 rounded-2xl border border-line p-4 {state_class}",
            onmousemove: move |_| {
                if let Some(active) = drag()
                    && active.list == hover_list
                    && active.index != index
                {
                    onreorder.call((active.index, index));
                    drag.set(Some(Drag {
                        list: hover_list.clone(),
                        index,
                    }));
                }
            },
            div { class: "flex items-center justify-between",
                Grip { list, index }
                Button {
                    variant: Variant::DangerGhost,
                    size: Size::Sm,
                    onclick: move |_| onremove.call(()),
                    "Remove"
                }
            }
            {children}
        }
    }
}

#[component]
pub fn Grip(#[props(into)] list: String, index: usize) -> Element {
    let mut drag = use_session().drag;

    rsx! {
        div {
            class: "press flex cursor-grab flex-col gap-[3px] rounded-lg px-2 py-2",
            onmousedown: move |_| {
                drag.set(Some(Drag {
                    list: list.clone(),
                    index,
                }));
            },
            for _ in 0..3 {
                div { class: "flex gap-[3px]",
                    span { class: "block size-1 rounded-full bg-soft" }
                    span { class: "block size-1 rounded-full bg-soft" }
                }
            }
        }
    }
}

#[component]
pub fn AddButton(#[props(into)] label: String, onclick: EventHandler<()>) -> Element {
    rsx! {
        Button {
            variant: Variant::Accent,
            size: Size::Sm,
            class: "self-start",
            onclick: move |_| onclick.call(()),
            "+ {label}"
        }
    }
}
