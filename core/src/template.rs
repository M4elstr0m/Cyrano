use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

mod error;
pub use error::Error;

mod store;
pub use store::Store;

mod rendered;
pub use rendered::RenderedTemplate;

pub(self) const AUTOESCAPE_SUFFIXES: [&str; 2] = ["html.tera", "html"];

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

        let mut template: Self = toml::from_str(&marker_content)?;

        if template.name.trim().is_empty() {
            return Err(Error::InvalidMarker);
        }

        template.entry_path = template_dir.join(&template.entry_path);
        if !template.entry_path.is_file() {
            return Err(Error::NoEntry);
        }
        if !has_autoescaped_suffix(&template.entry_path) {
            return Err(Error::InvalidEntry);
        }

        Ok(template)
    }
}

fn template_marker_path(template_dir: &Path) -> PathBuf {
    template_dir.join("TEMPLATE.cyrano")
}

fn has_autoescaped_suffix(path: &Path) -> bool {
    let name = path.to_string_lossy();
    AUTOESCAPE_SUFFIXES
        .iter()
        .any(|suffix| name.ends_with(suffix))
}
