use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::path::PathBuf;

mod error;
pub use error::Error;

const FILENAME: &str = "preferences.toml";
const APP_QUALIFIER: &str = "com";
const APP_ORGANIZATION: &str = "M4elstr0m";
const APP_NAME: &str = "Cyrano";

fn app_directory() -> Result<ProjectDirs, Error> {
    ProjectDirs::from(APP_QUALIFIER, APP_ORGANIZATION, APP_NAME).ok_or(Error::NoHomeDirectory)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    #[default]
    Dark,
}

impl Theme {
    pub fn canvas(self) -> &'static str {
        match self {
            Theme::Light => "#f4eed7",
            Theme::Dark => "#0a1322",
        }
    }

    pub fn class(self) -> &'static str {
        match self {
            Theme::Light => "",
            Theme::Dark => "dark",
        }
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Preferences {
    theme: Option<Theme>,

    profiles_dir: Option<PathBuf>,

    templates_dir: Option<PathBuf>,

    output_dir: Option<PathBuf>,
}

impl Preferences {
    fn path() -> Result<PathBuf, Error> {
        Ok(app_directory()?.config_dir().join(FILENAME))
    }

    pub fn load() -> Result<Self, Error> {
        match std::fs::read_to_string(Self::path()?) {
            Ok(s) => Ok(toml::from_str(&s)?),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Self::default()),
            Err(e) => Err(e.into()),
        }
    }

    pub fn save(&self) -> Result<(), Error> {
        let path = Self::path()?;

        std::fs::create_dir_all(path.parent().ok_or(Error::NoParentDirectory)?)?;

        std::fs::write(path, toml::to_string_pretty(self)?)?;

        Ok(())
    }

    pub fn theme(&self) -> Theme {
        self.theme.unwrap_or_default()
    }

    pub fn set_theme(&mut self, theme: Theme) {
        self.theme = Some(theme);
    }

    pub fn profiles_dir(&self) -> Result<PathBuf, Error> {
        match &self.profiles_dir {
            Some(p) => Ok(p.clone()),
            None => Ok(app_directory()?.data_dir().join("profiles")),
        }
    }

    pub fn templates_dir(&self) -> Result<PathBuf, Error> {
        match &self.templates_dir {
            Some(p) => Ok(p.clone()),
            None => Ok(app_directory()?.data_dir().join("templates")),
        }
    }

    pub fn output_dir(&self) -> Result<PathBuf, Error> {
        match &self.output_dir {
            Some(p) => Ok(p.clone()),
            None => Ok(app_directory()?.data_dir().join("output")),
        }
    }
}
