use crate::{
    components::{Button, Modal, TextInput, Tone, Variant},
    state::{ProfileEntry, use_app_state},
};
use cyrano_core::profile::Error;
use dioxus::prelude::*;
use std::path::PathBuf;

const MAX_ATTEMPTS: u8 = 3;

#[component]
pub fn UnlockDialog(
    entry: ProfileEntry,
    onunlocked: EventHandler<PathBuf>,
    onclose: EventHandler<()>,
) -> Element {
    let state = use_app_state();
    let mut password = use_signal(String::new);
    let mut attempts = use_signal(|| 0u8);
    let mut failure = use_signal(|| None::<String>);

    let submit = use_callback({
        let entry = entry.clone();

        move |_: ()| {
            let mut profiles = state.profiles;
            let result = profiles
                .write()
                .unlock_profile(&entry.dir, &password.read());

            match result {
                Ok(()) => onunlocked.call(entry.dir.clone()),
                Err(Error::Crypto(_)) => {
                    attempts.set(attempts() + 1);
                    password.set(String::new());

                    if attempts() >= MAX_ATTEMPTS {
                        state.notify(
                            Tone::Danger,
                            format!(
                                "Too many wrong passwords for {}. Select the profile again to retry.",
                                entry.name
                            ),
                        );
                        onclose.call(());
                    } else {
                        failure.set(Some(format!(
                            "Wrong password, {} attempt(s) left.",
                            MAX_ATTEMPTS - attempts()
                        )));
                    }
                }
                Err(error) => failure.set(Some(error.to_string())),
            }
        }
    });

    rsx! {
        Modal { title: "Unlock {entry.name}", onclose,
            p { class: "text-sm text-soft", "Enter the password to decrypt this profile." }
            TextInput {
                label: "Password",
                password: true,
                autofocus: true,
                value: password(),
                oninput: move |value| password.set(value),
                onenter: move |_| submit.call(()),
            }
            if let Some(message) = failure() {
                p { class: "text-sm text-danger", "{message}" }
            }
            div { class: "flex justify-end gap-3",
                Button {
                    variant: Variant::Ghost,
                    onclick: move |_| onclose.call(()),
                    "Cancel"
                }
                Button {
                    disabled: password().is_empty(),
                    onclick: move |_| submit.call(()),
                    "Unlock"
                }
            }
        }
    }
}
