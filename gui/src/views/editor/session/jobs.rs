use super::{Preview, Session};
use crate::{
    components::Tone,
    render::{self, Request},
};
use cyrano_core::profile::Asset;
use dioxus::prelude::*;

impl Session {
    pub fn save_asset(self, name: String, bytes: Vec<u8>) {
        let Some(dir) = self.active_dir() else { return };
        let asset = Asset::new(name, bytes);

        if asset.data_uri().is_err() {
            self.state
                .notify(Tone::Danger, "This image format is not supported.");
            return;
        }

        let result = self
            .state
            .profiles
            .read()
            .unlocked(&dir)
            .map(|profile| profile.save_asset(&dir, &asset));

        match result {
            Some(Ok(())) => self.refresh_assets(),
            Some(Err(error)) => self.state.fail(error),
            None => {}
        }
    }

    fn request(&self) -> Option<Request> {
        Some(Request {
            template_dir: self.template_dir.read().clone(),
            profile_dir: self.active_dir()?,
            cv: self.draft.read().clone(),
        })
    }

    pub fn run_preview(mut self) {
        let Some(request) = self.request() else { return };

        self.preview.set(Preview::Rendering);

        spawn(async move {
            self.preview
                .set(match render::preview(self.state, request).await {
                    Ok(pages) => Preview::Ready(pages),
                    Err(error) => Preview::Failed(error.to_string()),
                });
        });
    }

    pub fn run_export(mut self) {
        let Some(request) = self.request() else { return };

        self.exporting.set(true);

        spawn(async move {
            match render::export(self.state, request).await {
                Ok(path) => self.state.saved(&path),
                Err(error) => self.state.fail(error),
            }

            self.exporting.set(false);
        });
    }
}
