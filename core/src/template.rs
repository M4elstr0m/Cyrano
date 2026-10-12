use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use tera::Tera;

mod error;
pub use error::Error;

mod store;
pub use store::Store;

mod rendered;
pub use rendered::RenderedTemplate;

use crate::{cv_data::CvData, profile::Asset};

const AUTOESCAPE_SUFFIXES: [&str; 2] = ["html.tera", "html"];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Template {
    pub name: String,

    #[serde(default)]
    pub description: Option<String>,

    pub entry_path: PathBuf,

    #[serde(default)]
    pub required_assets: Vec<Asset>,
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

    pub fn render(
        &self,
        template_dir: &Path,
        cv_data: &CvData,
        assets: &[Asset],
    ) -> Result<RenderedTemplate, Error> {
        let glob = format!("{}/**/*", template_dir.display());
        let mut tera = Tera::new();
        tera.load_from_glob(&glob)?;
        tera.autoescape_on(AUTOESCAPE_SUFFIXES);

        let entry_name = self
            .entry_path
            .strip_prefix(template_dir)
            .map_err(|_| Error::InvalidEntry)?
            .to_string_lossy()
            .into_owned();

        let mut context = tera::Context::from_serialize(cv_data)?;

        for required_asset in &self.required_assets {
            if assets
                .iter()
                .filter(|a| a.name() == required_asset.name())
                .count()
                == 0
            {
                return Err(Error::MissingAsset(required_asset.name().to_owned()));
            }
        }

        for asset in assets {
            let uri = asset
                .data_uri()
                .map_err(|_| Error::BadAsset(asset.name().to_owned()))?;

            context.insert(asset.name().to_string(), &uri);
        }

        Ok(RenderedTemplate::new(tera.render(&entry_name, &context)?))
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
