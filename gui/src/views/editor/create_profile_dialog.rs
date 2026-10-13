use crate::{
    components::{Button, Modal, PasswordInput, TextInput, Variant},
    state::use_app_state,
};
use dioxus::prelude::*;
use std::path::PathBuf;

#[component]
pub fn CreateProfileDialog(oncreated: EventHandler<PathBuf>, onclose: EventHandler<()>) -> Element {
    let state = use_app_state();
    let mut name = use_signal(String::new);
    let mut password = use_signal(String::new);
    let mut confirmation = use_signal(String::new);

    let mismatch = !confirmation().is_empty() && password() != confirmation();
    let valid = !name().trim().is_empty() && !password().is_empty() && password() == confirmation();

    let submit = use_callback(move |_: ()| {
        if name().trim().is_empty() || password().is_empty() || password() != confirmation() {
            return;
        }

        let mut profiles = state.profiles;
        let result = profiles
            .write()
            .create_profile(name().trim().to_owned(), &password.read());

        match result {
            Ok(dir) => oncreated.call(dir),
            Err(error) => state.fail(error),
        }
    });

    rsx! {
        Modal { title: "Create a profile", onclose,
            p { class: "text-sm text-soft",
                "Your CV data is encrypted with this password. It cannot be recovered if you forget it."
            }
            TextInput {
                label: "Profile name",
                autofocus: true,
                value: name(),
                oninput: move |value| name.set(value),
            }
            PasswordInput {
                label: "Password",
                value: password(),
                oninput: move |value| password.set(value),
            }
            PasswordInput {
                label: "Confirm password",
                value: confirmation(),
                oninput: move |value| confirmation.set(value),
                onenter: move |_| submit.call(()),
            }
            if mismatch {
                p { class: "text-sm text-danger", "The passwords do not match." }
            }
            div { class: "flex justify-end gap-3",
                Button {
                    variant: Variant::Ghost,
                    onclick: move |_| onclose.call(()),
                    "Cancel"
                }
                Button { disabled: !valid, onclick: move |_| submit.call(()), "Create profile" }
            }
        }
    }
}
