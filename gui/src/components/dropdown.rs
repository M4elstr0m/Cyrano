use dioxus::prelude::*;

#[derive(Clone, Copy)]
struct Menu {
    open: Signal<bool>,
}

#[component]
pub fn Dropdown(trigger: Element, children: Element) -> Element {
    let mut open = use_signal(|| false);
    use_context_provider(|| Menu { open });

    rsx! {
        div { class: "relative",
            div { onclick: move |_| open.set(!open()), {trigger} }
            if open() {
                div {
                    class: "fixed inset-0 z-10",
                    onclick: move |_| open.set(false),
                }
                div { class: "absolute left-0 right-0 top-full z-20 mt-2 flex flex-col gap-1 rounded-2xl border border-line bg-surface p-2 shadow-overlay",
                    {children}
                }
            }
        }
    }
}

#[component]
pub fn DropdownItem(
    #[props(default)] onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    let mut menu = use_context::<Menu>();

    rsx! {
        button {
            class: "flex w-full items-center justify-between gap-3 rounded-xl px-3 py-2.5 text-left text-sm hover:bg-inset",
            r#type: "button",
            onclick: move |event| {
                menu.open.set(false);
                onclick.call(event);
            },
            {children}
        }
    }
}
