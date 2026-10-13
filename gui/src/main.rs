use components::ResizeHandles;
use dioxus::prelude::*;
use dioxus_native::{Config, LogicalSize, WindowAttributes};
use state::AppState;
use views::{Fatal, Shell};
use winit::{
    platform::{wayland::WindowAttributesExtWayland, x11::WindowAttributesExtX11},
    window::Icon,
};

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

fn window_icon() -> Option<Icon> {
    let rgba = image::load_from_memory(include_bytes!("../assets/logo-192.png"))
        .ok()?
        .into_rgba8();
    let (width, height) = rgba.dimensions();

    Icon::from_rgba(rgba.into_raw(), width, height).ok()
}

fn main() {
    let window = WindowAttributes::default()
        .with_title("Cyrano")
        .with_window_icon(window_icon())
        .with_decorations(false)
        .with_transparent(true)
        .with_inner_size(LogicalSize::new(1360.0, 860.0));
    let window = WindowAttributesExtWayland::with_name(window, "cyrano", "cyrano");
    let window = WindowAttributesExtX11::with_name(window, "cyrano", "cyrano");

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

    use_effect(move || {
        let title = match state.current_profile.read().as_deref() {
            Some(name) => format!("Cyrano - {name}"),
            None => "Cyrano".to_owned(),
        };

        document::document().set_title(title);
    });
    let frame = if (state.maximized)() {
        ""
    } else {
        "overflow-hidden rounded-2xl border border-line"
    };

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "preload", href: RALEWAY }
        document::Link { rel: "stylesheet", href: FONTS_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div { class: "{theme} {frame} relative h-screen w-screen bg-glow font-sans text-ink antialiased",
            Shell {}
            ResizeHandles {}
        }
    }
}
