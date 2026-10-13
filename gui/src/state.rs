use crate::{
    components::Tone,
    preferences::{Preferences, Theme},
};
use cyrano_core::{profile, template};
use dioxus::prelude::*;
use std::path::{Path, PathBuf};

mod error;
pub use error::Error;

#[derive(Clone, PartialEq)]
pub struct Notice {
    pub tone: Tone,
    pub message: String,
    pub reveal: Option<PathBuf>,
}

#[derive(Clone, Copy, PartialEq)]
pub struct AppState {
    pub preferences: Signal<Preferences>,
    pub templates: Signal<template::Store>,
    pub profiles: Signal<profile::Store>,
    pub notices: Signal<Vec<Notice>>,
}

pub fn use_app_state() -> AppState {
    use_context()
}

fn describe<E: std::fmt::Display>((path, error): (PathBuf, E)) -> Notice {
    Notice {
        tone: Tone::Danger,
        message: format!("{}: {error}", path.display()),
        reveal: None,
    }
}

impl AppState {
    pub fn load() -> Result<Self, Error> {
        let preferences = Preferences::load()?;

        let (templates, template_warnings) = template::Store::load(&preferences.templates_dir()?)?;
        let (profiles, profile_warnings) = profile::Store::load(&preferences.profiles_dir()?)?;

        let notices = template_warnings
            .into_iter()
            .map(describe)
            .chain(profile_warnings.into_iter().map(describe))
            .collect();

        Ok(Self {
            preferences: Signal::new(preferences),
            templates: Signal::new(templates),
            profiles: Signal::new(profiles),
            notices: Signal::new(notices),
        })
    }

    pub fn theme(&self) -> Theme {
        self.preferences.read().theme()
    }

    pub fn set_theme(mut self, theme: Theme) {
        self.preferences.write().set_theme(theme);

        if let Err(error) = self.preferences.read().save() {
            self.fail(error);
        }
    }

    pub fn template_dir(&self, name: &str) -> Option<PathBuf> {
        Some(self.preferences.read().templates_dir().ok()?.join(name))
    }

    pub fn notify(mut self, tone: Tone, message: impl Into<String>) {
        self.notices.write().push(Notice {
            tone,
            message: message.into(),
            reveal: None,
        });
    }

    pub fn fail(self, error: impl std::fmt::Display) {
        self.notify(Tone::Danger, error.to_string());
    }

    pub fn saved(mut self, path: &Path) {
        self.notices.write().push(Notice {
            tone: Tone::Success,
            message: format!("CV saved to {}", path.display()),
            reveal: Some(path.to_path_buf()),
        });
    }

    pub fn dismiss(mut self, index: usize) {
        let mut notices = self.notices.write();

        if index < notices.len() {
            notices.remove(index);
        }
    }
}
