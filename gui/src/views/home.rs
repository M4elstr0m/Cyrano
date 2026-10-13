use crate::{
    components::{Badge, Button, Tone, Variant},
    route::Route,
    state::use_app_state,
    text::humanize,
};
use dioxus::prelude::*;

mod empty;
use empty::EmptyTemplates;

#[component]
pub fn Home() -> Element {
    let state = use_app_state();
    let templates = state.templates.read();

    let mut entries = templates.all_templates();
    entries.sort_by(|a, b| a.1.name.cmp(&b.1.name));

    rsx! {
        div { class: "h-full overflow-y-auto",
            div { class: "mx-auto flex max-w-6xl flex-col gap-10 px-8 py-12",
                div { class: "flex items-end justify-between gap-6",
                    div { class: "flex flex-col gap-2",
                        h1 { class: "text-4xl font-semibold tracking-tight", "Choose a template" }
                        p { class: "max-w-xl text-base text-soft",
                            "Pick a layout for your CV, then fill in your profile and preview the result before exporting it."
                        }
                    }
                    Button {
                        variant: Variant::Secondary,
                        onclick: move |_| state.reload_templates(),
                        "Rescan"
                    }
                }
                if entries.is_empty() {
                    EmptyTemplates {}
                } else {
                    div { class: "flex flex-wrap gap-6",
                        for (dir , template) in entries {
                            TemplateCard {
                                key: "{dir.display()}",
                                folder: dir.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default(),
                                name: template.name.clone(),
                                description: template.description.clone(),
                                assets: template.required_assets.iter().map(|asset| asset.name().to_owned()).collect::<Vec<_>>(),
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TemplateCard(
    folder: String,
    name: String,
    description: Option<String>,
    assets: Vec<String>,
) -> Element {
    let navigator = use_navigator();
    let initial: String = name.chars().take(1).flat_map(char::to_uppercase).collect();

    rsx! {
        div {
            class: "flex w-80 cursor-pointer flex-col gap-5 rounded-3xl border border-line bg-surface p-6 shadow-card hover:border-accent",
            onclick: move |_| {
                navigator
                    .push(Route::Editor {
                        template: folder.clone(),
                    });
            },
            span { class: "flex size-12 items-center justify-center rounded-2xl bg-accent-soft text-xl font-semibold text-accent",
                "{initial}"
            }
            div { class: "flex flex-col gap-1.5",
                h2 { class: "text-lg font-semibold", "{name}" }
                if let Some(description) = description {
                    p { class: "text-sm text-soft", "{description}" }
                }
            }
            if !assets.is_empty() {
                div { class: "flex flex-wrap items-center gap-2",
                    for asset in assets {
                        Badge { key: "{asset}", tone: Tone::Accent, "{humanize(&asset)}" }
                    }
                }
            }
            span { class: "mt-auto self-start rounded-full bg-primary px-4 py-2 text-sm font-medium text-on-primary",
                "Use template"
            }
        }
    }
}
