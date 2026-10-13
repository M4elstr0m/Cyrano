use dioxus::prelude::*;
use dioxus_native::{Config, LogicalSize, WindowAttributes};
use state::AppState;
use views::{Fatal, Shell};

mod components;
mod preferences;
mod render;
mod state;
mod text;
mod views;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
const FONTS_CSS: Asset = asset!("/assets/fonts.css");
const RALEWAY: Asset = asset!(
    "/assets/fonts/Raleway.ttf",
    AssetOptions::builder().with_hash_suffix(false)
);

fn main() {
    let window = WindowAttributes::default()
        .with_title("Cyrano")
        .with_decorations(false)
        .with_inner_size(LogicalSize::new(1360.0, 860.0));

    dioxus_native::launch_cfg(
        App,
        Vec::new(),
        vec![Box::new(Config::new().with_window_attributes(window))],
    );
}

#[component]
fn App() -> Element {
    let state = use_hook(|| AppState::load().map_err(|error| error.to_string()));

    match state {
        Ok(state) => rsx! {
            Root { state }
        },
        Err(message) => rsx! {
            Fatal { message }
        },
    }
}

#[component]
fn Root(state: AppState) -> Element {
    use_context_provider(|| state);
    let theme = state.theme().class();

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "preload", href: RALEWAY }
        document::Link { rel: "stylesheet", href: FONTS_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "{theme} h-screen w-screen bg-glow font-sans text-ink antialiased",
            Shell {}
        }
    }
}
