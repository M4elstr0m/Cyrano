use crate::state::use_app_state;
use dioxus::{html::input_data::MouseButton, prelude::*};
use std::sync::Arc;
use winit::window::{ResizeDirection, Window};

fn use_window() -> Arc<Window> {
    use_context()
}

#[component]
pub fn DragRegion() -> Element {
    let window = use_window();
    let toggle = window.clone();
    let mut maximized = use_app_state().maximized;

    rsx! {
        div {
            class: "h-full flex-1",
            onmousedown: move |event| {
                if event.trigger_button() == Some(MouseButton::Primary) {
                    let _ = window.drag_window();
                }
            },
            ondoubleclick: move |_| {
                let next = !maximized();
                toggle.set_maximized(next);
                maximized.set(next);
            },
        }
    }
}

#[component]
pub fn WindowControls() -> Element {
    let window = use_window();
    let state = use_app_state();
    let minimize = window.clone();
    let mut maximized = state.maximized;
    let observed = window.clone();

    use_future(move || {
        let observed = observed.clone();

        async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_millis(250)).await;

                let current = observed.is_maximized();

                if current != *maximized.peek() {
                    maximized.set(current);
                }
            }
        }
    });

    rsx! {
        div { class: "flex items-center gap-1",
            ControlButton { onclick: move |_| minimize.set_minimized(true),
                span { class: "block h-0.5 w-3.5 rounded-full bg-current" }
            }
            ControlButton {
                onclick: move |_| {
                    let next = !maximized();
                    window.set_maximized(next);
                    maximized.set(next);
                },
                if maximized() {
                    div { class: "relative size-3.5",
                        span { class: "absolute right-0 top-0 block size-2.5 rounded-[2px] border-2 border-current" }
                        span { class: "absolute bottom-0 left-0 block size-2.5 rounded-[2px] border-2 border-current bg-surface" }
                    }
                } else {
                    span { class: "block size-3.5 rounded-[3px] border-2 border-current" }
                }
            }
            ControlButton { danger: true, onclick: move |_| state.quit(),
                svg {
                    width: "14",
                    height: "14",
                    view_box: "0 0 14 14",
                    fill: "none",
                    path {
                        d: "M1 1 L13 13 M13 1 L1 13",
                        stroke: "currentColor",
                        stroke_width: "2.4",
                        stroke_linecap: "round",
                    }
                }
            }
        }
    }
}

#[component]
fn ControlButton(
    #[props(default)] danger: bool,
    onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    let hover = if danger {
        "hover:bg-danger hover:text-white"
    } else {
        "hover:bg-accent-soft hover:text-accent"
    };

    rsx! {
        button {
            class: "control flex size-9 items-center justify-center rounded-full text-soft {hover}",
            r#type: "button",
            onclick: move |event| onclick.call(event),
            {children}
        }
    }
}

#[component]
pub fn ResizeHandles() -> Element {
    rsx! {
        Handle {
            direction: ResizeDirection::North,
            class: "left-3 right-3 top-0 h-1 cursor-ns-resize",
        }
        Handle {
            direction: ResizeDirection::South,
            class: "bottom-0 left-3 right-3 h-1 cursor-ns-resize",
        }
        Handle {
            direction: ResizeDirection::West,
            class: "bottom-3 left-0 top-3 w-1 cursor-ew-resize",
        }
        Handle {
            direction: ResizeDirection::East,
            class: "bottom-3 right-0 top-3 w-1 cursor-ew-resize",
        }
        Handle {
            direction: ResizeDirection::NorthWest,
            class: "left-0 top-0 size-3 cursor-nwse-resize",
        }
        Handle {
            direction: ResizeDirection::NorthEast,
            class: "right-0 top-0 size-3 cursor-nesw-resize",
        }
        Handle {
            direction: ResizeDirection::SouthWest,
            class: "bottom-0 left-0 size-3 cursor-nesw-resize",
        }
        Handle {
            direction: ResizeDirection::SouthEast,
            class: "bottom-0 right-0 size-3 cursor-nwse-resize",
        }
    }
}

#[component]
fn Handle(direction: ResizeDirection, class: String) -> Element {
    let window = use_window();

    rsx! {
        div {
            class: "absolute z-50 {class}",
            onmousedown: move |event| {
                if event.trigger_button() == Some(MouseButton::Primary) {
                    let _ = window.drag_resize_window(direction);
                }
            },
        }
    }
}
