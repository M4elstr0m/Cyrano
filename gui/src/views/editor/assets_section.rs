use super::session::{AssetEntry, use_session};
use crate::{
    components::{Badge, Button, Section, Size, Tone, Variant},
    text::humanize,
};
use dioxus::prelude::*;

const IMAGE_EXTENSIONS: [&str; 7] = ["png", "jpg", "jpeg", "webp", "gif", "svg", "bmp"];

#[component]
pub fn AssetsSection() -> Element {
    let session = use_session();
    let assets = session.assets.read();

    if assets.is_empty() {
        return rsx! {};
    }

    rsx! {
        Section { title: "Images", open: true,
            for asset in assets.iter() {
                AssetRow { key: "{asset.name}", asset: asset.clone() }
            }
        }
    }
}

#[component]
fn AssetRow(asset: AssetEntry) -> Element {
    let session = use_session();
    let label = humanize(&asset.name);
    let action = if asset.uri.is_some() {
        "Replace"
    } else {
        "Upload"
    };
    let name = asset.name.clone();

    let pick = move |_| {
        let name = name.clone();

        spawn(async move {
            let picked = rfd::AsyncFileDialog::new()
                .add_filter("Images", &IMAGE_EXTENSIONS)
                .pick_file()
                .await;

            if let Some(file) = picked {
                session.save_asset(name, file.read().await);
            }
        });
    };

    rsx! {
        div { class: "flex items-center gap-4",
            if let Some(uri) = &asset.uri {
                img {
                    class: "size-14 shrink-0 rounded-2xl border border-line bg-inset",
                    src: uri.as_str(),
                }
            } else {
                div { class: "size-14 shrink-0 rounded-2xl border border-line bg-inset" }
            }
            div { class: "flex min-w-0 flex-1 flex-col gap-1",
                span { class: "truncate text-sm font-medium", "{label}" }
                if asset.uri.is_some() {
                    Badge { tone: Tone::Success, "Ready" }
                } else {
                    Badge { tone: Tone::Danger, "Missing" }
                }
            }
            Button {
                variant: Variant::Secondary,
                size: Size::Sm,
                onclick: pick,
                "{action}"
            }
        }
    }
}
