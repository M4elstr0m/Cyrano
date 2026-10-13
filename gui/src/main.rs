use dioxus::{
    native::{Config, LogicalSize, WindowAttributes},
    prelude::*,
};
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

fn main() {
    let window = WindowAttributes::default()
        .with_title("Cyrano")
        .with_inner_size(LogicalSize::new(1360.0, 860.0));

    dioxus::native::launch_cfg(
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
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "{theme} h-screen w-screen bg-canvas font-sans text-ink antialiased",
            Shell {}
        }
    }
}
