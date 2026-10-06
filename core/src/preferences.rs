use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::path::PathBuf;

mod error;
pub use error::Error;

const FILENAME: &str = "preferences.toml";

fn app_directory() -> Result<ProjectDirs, Error> {
    ProjectDirs::from(
        crate::APP_QUALIFIER,
        crate::APP_ORGANIZATION,
        crate::APP_NAME,
    )
    .ok_or(Error::NoHomeDirectory)
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Preferences {
    profiles_path: Option<PathBuf>,
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

    pub fn profiles_path(&self) -> Result<PathBuf, Error> {
        match &self.profiles_path {
            Some(p) => Ok(p.clone()),
            None => Ok(app_directory()?.data_dir().join("profiles")),
        }
    }
}
