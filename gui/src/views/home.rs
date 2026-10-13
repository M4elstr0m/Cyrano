use crate::{
    components::{Badge, Button, ConfirmDialog, Logo, Size, Tone, Variant},
    state::use_app_state,
    text::humanize,
};
use dioxus::prelude::*;
use std::path::PathBuf;

mod empty;
use empty::EmptyTemplates;

#[component]
pub fn Home() -> Element {
    let state = use_app_state();
    let mut pending = use_signal(|| None::<(PathBuf, String)>);
    let templates = state.templates.read();

    let mut entries = templates.all_templates();
    entries.sort_by(|a, b| a.1.name.cmp(&b.1.name));

    rsx! {
        div { class: "h-full w-full overflow-y-auto",
            div { class: "mx-auto flex max-w-6xl flex-col gap-10 px-8 py-12",
                div { class: "flex items-end justify-between gap-6",
                    div { class: "flex items-center gap-6",
                        Logo { class: "animate-float size-24", large: true }
                        div { class: "animate-fade-up flex flex-col gap-2",
                            h1 { class: "text-4xl font-bold tracking-tight", "Choose a template" }
                            p { class: "max-w-xl text-base text-soft",
                                "Pick a layout for your CV, then fill in your profile and preview the result before exporting it."
                            }
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
                        for (dir, template) in entries {
                            TemplateCard {
                                key: "{dir.display()}",
                                folder: dir.file_name().map(|name| name.to_string_lossy().into_owned()).unwrap_or_default(),
                                name: template.name.clone(),
                                description: template.description.clone(),
                                assets: template
                                                                    .required_assets
                                                                    .iter()
                                                                    .map(|asset| asset.to_owned())
                                                                    .collect::<Vec<_>>(),
                                ondelete: {
                                    let dir = dir.to_path_buf();
                                    let name = template.name.clone();
                                    move |_| pending.set(Some((dir.clone(), name.clone())))
                                },
                            }
                        }
                    }
                }
            }
        }
        if let Some((dir, name)) = pending() {
            ConfirmDialog {
                title: "Delete {name}?",
                message: "The template folder will be permanently deleted. This cannot be undone.",
                confirm_label: "Delete template",
                onconfirm: move |_| {
                    pending.set(None);
                    state.delete_template(&dir);
                },
                onclose: move |_| pending.set(None),
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
    ondelete: EventHandler<()>,
) -> Element {
    let state = use_app_state();
    let initial: String = name.chars().take(1).flat_map(char::to_uppercase).collect();

    rsx! {
        div {
            class: "animate-fade-up grow flex w-80 cursor-pointer flex-col rounded-[28px] border border-line bg-surface p-4 shadow-card",
            onclick: move |_| state.open_template(folder.clone()),
            div { class: "bg-sheen flex h-44 items-center justify-center rounded-2xl border border-line",
                div { class: "flex h-32 w-24 flex-col gap-2 rounded-lg bg-white p-3 shadow-page",
                    div { class: "flex items-center gap-2",
                        span { class: "bg-brand size-5 rounded-full" }
                        span { class: "h-1.5 flex-1 rounded-full bg-[#d9deea]" }
                    }
                    span { class: "h-1.5 w-full rounded-full bg-[#e6e9f2]" }
                    span { class: "h-1.5 w-4/5 rounded-full bg-[#e6e9f2]" }
                    span { class: "h-1.5 w-3/5 rounded-full bg-[#e6e9f2]" }
                    span { class: "mt-auto text-[10px] font-bold text-[#2f6bff]", "{initial}" }
                }
            }
            div { class: "mt-5 flex flex-col gap-1.5 px-2",
                h2 { class: "text-lg font-semibold tracking-tight", "{name}" }
                if let Some(description) = description {
                    p { class: "text-sm text-soft", "{description}" }
                }
            }
            if !assets.is_empty() {
                div { class: "mt-4 flex flex-wrap items-center gap-2 px-2",
                    for asset in assets {
                        Badge { key: "{asset}", tone: Tone::Accent, "{humanize(&asset)}" }
                    }
                }
            }
            div { class: "mt-auto flex items-center justify-between gap-3 px-2 pb-2 pt-5",
                Button { "Use template" }
                Button {
                    variant: Variant::Ghost,
                    size: Size::Sm,
                    onclick: move |event: MouseEvent| {
                        event.stop_propagation();
                        ondelete.call(());
                    },
                    "Delete"
                }
            }
        }
    }
}
