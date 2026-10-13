use dioxus::prelude::*;

#[derive(Clone, Copy)]
struct Menu {
    open: Signal<bool>,
}

#[component]
pub fn Dropdown(#[props(default)] class: String, trigger: Element, children: Element) -> Element {
    let mut open = use_signal(|| false);
    use_context_provider(|| Menu { open });

    rsx! {
        div { class: "flex flex-col gap-2 {class}",
            div { class: "flex flex-col", onclick: move |_| open.set(!open()), {trigger} }
            if open() {
                div { class: "animate-fade-up flex flex-col gap-1 rounded-2xl border border-line bg-surface p-2",
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
            class: "flex w-full items-center justify-between gap-3 fade-hover rounded-xl px-3 py-2.5 text-left text-sm hover:bg-inset",
            r#type: "button",
            onclick: move |event| {
                menu.open.set(false);
                onclick.call(event);
            },
            {children}
        }
    }
}
