use dioxus::prelude::*;

#[component]
pub fn Modal(
    #[props(into)] title: String,
    onclose: EventHandler<()>,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: "fixed inset-0 z-50 flex items-center justify-center bg-[rgba(5,7,12,0.6)] p-6",
            onclick: move |_| onclose.call(()),
            div {
                class: "flex w-full max-w-md flex-col gap-5 rounded-3xl border border-line bg-surface p-6 shadow-overlay",
                onclick: move |event| event.stop_propagation(),
                div { class: "flex items-center justify-between",
                    h2 { class: "text-lg font-semibold", "{title}" }
                    button {
                        class: "rounded-full px-2.5 py-1 text-lg leading-none text-soft hover:bg-inset",
                        r#type: "button",
                        onclick: move |_| onclose.call(()),
                        "×"
                    }
                }
                {children}
            }
        }
    }
}
