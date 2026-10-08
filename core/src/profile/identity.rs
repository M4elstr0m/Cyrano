use super::Error;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub name: String,

    #[serde(default)]
    pub created_at: Option<i64>,

    #[serde(default)]
    pub last_interacted_with: Option<i64>,

    #[serde(default)]
    pub color: Option<String>, // hex color for ui
}

impl Identity {
    pub fn load(profile_dir: &Path) -> Result<Self, Error> {
        let marker_content = match std::fs::read_to_string(super::identity_marker_path(profile_dir))
        {
            Ok(v) => v,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Err(Error::NoIdentity),
            Err(e) => return Err(e.into()),
        };

        let identity: Self = toml::from_str(&marker_content)?;

        if identity.name.trim().is_empty() {
            return Err(Error::InvalidIdentity);
        }

        Ok(identity)
    }

    pub fn save(&self, profile_dir: &Path) -> Result<(), Error> {
        let marker_content = toml::to_string_pretty(self)?;

        std::fs::write(super::identity_marker_path(profile_dir), marker_content)?;
        Ok(())
    }
}
