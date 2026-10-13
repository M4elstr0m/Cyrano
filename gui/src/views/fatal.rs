use crate::components::Card;
use dioxus::prelude::*;

#[component]
pub fn Fatal(message: String) -> Element {
    rsx! {
        div { class: "dark flex h-screen w-screen items-center justify-center bg-canvas p-8 font-sans text-ink",
            Card { class: "flex max-w-lg flex-col gap-3 p-8",
                h1 { class: "text-xl font-semibold", "Cyrano could not start" }
                p { class: "break-words text-sm text-soft", "{message}" }
            }
        }
    }
}
