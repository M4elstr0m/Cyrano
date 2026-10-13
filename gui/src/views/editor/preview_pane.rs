use super::session::{Preview, use_session};
use crate::components::{Card, Logo, Tone};
use dioxus::prelude::*;

#[component]
pub fn PreviewPane() -> Element {
    let session = use_session();
    let preview = session.preview.read();

    rsx! {
        section { class: "min-w-0 flex-1 overflow-y-auto",
            match &*preview {
                Preview::Idle => rsx! {
                    Placeholder {
                        title: "Nothing to preview yet",
                        detail: "Select a profile, then press Preview to see your CV rendered here.",
                    }
                },
                Preview::Rendering => rsx! {
                    Placeholder { title: "Rendering your CV", detail: "This usually takes a few seconds." }
                },
                Preview::Failed(message) => rsx! {
                    Placeholder { title: "Preview failed", detail: message.clone(), tone: Tone::Danger }
                },
                Preview::Ready(pages) => rsx! {
                    div { class: "mx-auto flex max-w-[860px] flex-col items-center gap-8 p-10",
                        for (index, page) in pages.iter().enumerate() {
                            img {
                                key: "{index}",
                                class: "animate-fade-up block w-full rounded-lg shadow-page",
                                src: page.as_str(),
                            }
                        }
                    }
                },
            }
        }
    }
}

#[component]
fn Placeholder(title: String, detail: String, #[props(default)] tone: Tone) -> Element {
    let tone_class = tone.text_class();

    rsx! {
        div { class: "flex h-full items-center justify-center p-10",
            Card { class: "flex max-w-sm flex-col items-center gap-3 p-8 text-center",
                Logo { class: "size-20", large: true }
                h2 { class: "text-lg font-semibold {tone_class}", "{title}" }
                p { class: "break-words text-sm text-soft", "{detail}" }
            }
        }
    }
}
