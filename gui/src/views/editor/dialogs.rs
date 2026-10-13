use super::{
    create_profile_dialog::CreateProfileDialog, session::use_session, unlock_dialog::UnlockDialog,
};
use crate::components::ConfirmDialog;
use dioxus::prelude::*;

#[component]
pub fn Dialogs() -> Element {
    let session = use_session();
    let mut unlocking = session.unlocking;
    let mut creating = session.creating;
    let mut deleting = session.deleting;

    rsx! {
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
        if deleting() {
            ConfirmDialog {
                title: "Delete this profile?",
                message: "The profile and its encrypted data will be permanently deleted. This cannot be undone.",
                confirm_label: "Delete profile",
                onconfirm: move |_| {
                    deleting.set(false);
                    session.delete_active();
                },
                onclose: move |_| deleting.set(false),
            }
        }
    }
}
