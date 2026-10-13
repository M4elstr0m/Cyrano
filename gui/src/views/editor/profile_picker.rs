use super::{
    create_profile_dialog::CreateProfileDialog, session::use_session, unlock_dialog::UnlockDialog,
};
use crate::{
    components::{Badge, Button, Dropdown, DropdownItem, Size, Tone, Variant},
    state::{ProfileEntry, use_app_state},
};
use dioxus::prelude::*;

#[component]
pub fn ProfilePicker() -> Element {
    let state = use_app_state();
    let session = use_session();
    let mut unlocking = use_signal(|| None::<ProfileEntry>);
    let mut creating = use_signal(|| false);

    let entries = state.profile_entries();
    let active = session.active_dir();
    let label = entries
        .iter()
        .find(|entry| Some(&entry.dir) == active.as_ref())
        .map(|entry| entry.name.clone())
        .unwrap_or_else(|| "Select a profile".to_owned());

    let pick = use_callback(move |entry: ProfileEntry| {
        if entry.unlocked {
            session.select(entry.dir);
        } else {
            unlocking.set(Some(entry));
        }
    });

    rsx! {
        div { class: "flex flex-col gap-2",
            span { class: "text-xs font-medium text-soft", "Profile" }
            div { class: "flex items-center gap-2",
                div { class: "min-w-0 flex-1",
                    Dropdown {
                        trigger: rsx! {
                            div { class: "flex cursor-pointer items-center justify-between gap-3 rounded-2xl border border-line bg-inset px-4 py-3 hover:border-accent",
                                span { class: "truncate text-sm font-medium", "{label}" }
                                span { class: "text-xs text-soft", "Change" }
                            }
                        },
                        for entry in entries {
                            DropdownItem {
                                key: "{entry.dir.display()}",
                                onclick: {
                                    let entry = entry.clone();
                                    move |_| pick.call(entry.clone())
                                },
                                span { class: "truncate", "{entry.name}" }
                                if entry.unlocked {
                                    Badge { tone: Tone::Success, "Unlocked" }
                                } else {
                                    Badge { "Locked" }
                                }
                            }
                        }
                        DropdownItem { onclick: move |_| creating.set(true),
                            span { class: "font-medium text-accent", "Create a profile" }
                        }
                    }
                }
                if active.is_some() {
                    Button {
                        variant: Variant::Secondary,
                        size: Size::Sm,
                        onclick: move |_| session.lock(),
                        "Lock"
                    }
                }
            }
        }
        if let Some(entry) = unlocking() {
            UnlockDialog {
                entry,
                onunlocked: move |dir| {
                    unlocking.set(None);
                    session.select(dir);
                },
                onclose: move |_| unlocking.set(None),
            }
        }
        if creating() {
            CreateProfileDialog {
                oncreated: move |dir| {
                    creating.set(false);
                    session.select(dir);
                },
                onclose: move |_| creating.set(false),
            }
        }
    }
}
