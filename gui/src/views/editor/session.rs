use crate::state::{AppState, ProfileEntry};
use cyrano_core::cv_data::CvData;
use dioxus::prelude::*;
use std::{path::PathBuf, time::Duration};

mod jobs;
mod profile;

const AUTOSAVE_DELAY: Duration = Duration::from_millis(700);

#[derive(Clone, PartialEq, Default)]
pub enum Preview {
    #[default]
    Idle,
    Rendering,
    Ready(Vec<String>),
    Failed(String),
}

#[derive(Clone, PartialEq)]
pub struct AssetEntry {
    pub name: String,
    pub uri: Option<String>,
}

#[derive(Clone, Copy)]
pub struct Session {
    pub state: AppState,
    pub template_dir: Signal<PathBuf>,
    pub active: Signal<Option<PathBuf>>,
    pub draft: Signal<CvData>,
    pub assets: Signal<Vec<AssetEntry>>,
    pub preview: Signal<Preview>,
    pub exporting: Signal<bool>,
    pub unlocking: Signal<Option<ProfileEntry>>,
    pub creating: Signal<bool>,
    pub deleting: Signal<bool>,
}

pub fn use_session() -> Session {
    use_context()
}

pub fn use_autosave(session: Session) {
    use_resource(move || async move {
        drop(session.draft.read());
        tokio::time::sleep(AUTOSAVE_DELAY).await;
        session.persist();
    });

    use_drop(move || session.persist());
}

impl Session {
    pub fn new(state: AppState, template_dir: PathBuf) -> Self {
        Self {
            state,
            template_dir: Signal::new(template_dir),
            active: Signal::new(None),
            draft: Signal::new(CvData::new(String::new())),
            assets: Signal::new(Vec::new()),
            preview: Signal::new(Preview::Idle),
            exporting: Signal::new(false),
            unlocking: Signal::new(None),
            creating: Signal::new(false),
            deleting: Signal::new(false),
        }
    }

    pub fn template_name(&self) -> String {
        self.state
            .templates
            .read()
            .template(&self.template_dir.read())
            .map(|template| template.name.clone())
            .unwrap_or_default()
    }

    pub fn required_assets(&self) -> Vec<String> {
        self.state
            .templates
            .read()
            .template(&self.template_dir.read())
            .map(|template| {
                template
                    .required_assets
                    .iter()
                    .map(|asset| asset.name().to_owned())
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn active_dir(&self) -> Option<PathBuf> {
        self.active.read().clone()
    }

    pub fn missing_assets(&self) -> bool {
        self.assets.read().iter().any(|asset| asset.uri.is_none())
    }

    pub fn busy(&self) -> bool {
        *self.preview.read() == Preview::Rendering || *self.exporting.read()
    }

    pub fn blocker(&self) -> Option<&'static str> {
        if self.active.read().is_none() {
            Some("Select a profile to continue.")
        } else if self.missing_assets() {
            Some("Upload the required images to continue.")
        } else {
            None
        }
    }

    pub fn persist(self) {
        let (Ok(active), Ok(draft)) = (self.active.try_read(), self.draft.try_read()) else {
            return;
        };
        let Some(dir) = active.as_ref() else { return };

        let mut profiles = self.state.profiles;
        let mut store = profiles.write();
        let Some(profile) = store.unlocked_mut(dir) else {
            return;
        };

        *profile.data_mut() = draft.clone();

        if profile.is_modified().unwrap_or(true)
            && let Err(error) = profile.save(dir)
        {
            self.state.fail(error);
        }
    }
}
