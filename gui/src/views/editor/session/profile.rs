use super::{AssetEntry, Preview, Session};
use cyrano_core::cv_data::CvData;
use dioxus::prelude::*;
use std::path::PathBuf;

impl Session {
    pub fn select(mut self, dir: PathBuf) {
        self.persist();

        let data = self
            .state
            .profiles
            .read()
            .unlocked(&dir)
            .map(|profile| profile.data().clone());

        let Some(data) = data else { return };

        self.draft.set(data);
        self.active.set(Some(dir));
        self.preview.set(Preview::Idle);
        self.refresh_assets();
    }

    pub fn lock(mut self) {
        self.persist();

        let Some(dir) = self.active_dir() else { return };
        let mut profiles = self.state.profiles;
        let result = profiles.write().lock_profile(&dir);

        if let Err(error) = result {
            self.state.fail(error);
            return;
        }

        self.active.set(None);
        self.draft.set(CvData::new(String::new()));
        self.assets.set(Vec::new());
        self.preview.set(Preview::Idle);
    }

    pub fn refresh_assets(mut self) {
        let entries = match self.active_dir() {
            Some(dir) => {
                let profiles = self.state.profiles.read();
                let profile = profiles.unlocked(&dir);

                self.required_assets()
                    .into_iter()
                    .map(|name| {
                        let uri = profile
                            .and_then(|profile| profile.load_asset(&dir, &name).ok())
                            .and_then(|asset| asset.data_uri().ok());

                        AssetEntry { name, uri }
                    })
                    .collect()
            }
            None => Vec::new(),
        };

        self.assets.set(entries);
    }
}
