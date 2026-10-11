use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

mod error;
pub use error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub name: String,

    #[serde(default)]
    pub description: Option<String>,

    pub entry_path: PathBuf,
}

impl Template {
    pub fn load(template_dir: &Path) -> Result<Self, Error> {
        let marker_content = match std::fs::read_to_string(template_marker_path(template_dir)) {
            Ok(v) => v,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Err(Error::NoMarker),
            Err(e) => return Err(e.into()),
        };

        let template: Self = toml::from_str(&marker_content)?;

        if template.name.trim().is_empty() {
            return Err(Error::InvalidMarker);
        }

        Ok(template)
    }
}

fn template_marker_path(profile_dir: &Path) -> PathBuf {
    profile_dir.join("TEMPLATE.cyrano")
}
