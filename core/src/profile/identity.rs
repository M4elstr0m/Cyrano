use crate::crypto;

use super::Error;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub name: String,

    pub created_at: u64,

    #[serde(default)]
    pub last_interacted_with: Option<u64>,

    #[serde(default)]
    pub color: Option<String>, // hex color for ui
}

impl Identity {
    pub fn new(name: String) -> Result<Self, Error> {
        let current_time = crate::time::current_timestamp()?;

        Ok(Self {
            name,
            created_at: current_time,
            last_interacted_with: Some(current_time),
            color: None,
        })
    }

    pub(super) fn generate_folder_name(&self) -> Result<String, Error> {
        let sanitized_name = crate::path::sanitize_filename(&self.name);

        let mut buf = [0u8; 1];
        getrandom::fill(&mut buf).map_err(crypto::Error::Random)?;
        let rand_suffix = format!("{:02x}", buf[0]);

        Ok(format!(
            "{sanitized_name}-{}-{rand_suffix}",
            self.created_at
        ))
    }

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
