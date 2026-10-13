use super::Tone;
use dioxus::prelude::*;

#[component]
pub fn Badge(#[props(default)] tone: Tone, children: Element) -> Element {
    let tone_class = tone.badge_class();

    rsx! {
        span { class: "inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium {tone_class}",
            {children}
        }
    }
}
