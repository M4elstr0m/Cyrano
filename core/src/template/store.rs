use tera::Tera;

use crate::cv_data::CvData;

use super::{Error, RenderedTemplate, Template};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub struct Store {
    templates: HashMap<PathBuf, Template>,
}

impl Store {
    pub fn load(templates_dir: &Path) -> Result<(Self, Vec<(PathBuf, Error)>), Error> {
        std::fs::create_dir_all(templates_dir)?;

        let mut templates: HashMap<PathBuf, Template> = HashMap::new();

        let mut marker_warnings: Vec<(PathBuf, Error)> = Vec::new();

        for entry in std::fs::read_dir(templates_dir)? {
            let template_dir = entry?.path();

            if !template_dir.is_dir() {
                continue;
            }

            match Template::load(&template_dir) {
                Ok(t) => {
                    templates.insert(template_dir, t);
                }
                Err(e) => marker_warnings.push((template_dir, e)),
            }
        }

        Ok((Store { templates }, marker_warnings))
    }

    pub fn template(&self, template_dir: &Path) -> Option<&Template> {
        self.templates.get(template_dir)
    }

    pub fn all_templates(&self) -> Vec<(&Path, &Template)> {
        self.templates
            .iter()
            .map(|(path, template)| (path.as_path(), template))
            .collect()
    }

    pub fn render(&self, template_dir: &Path, cv_data: &CvData) -> Result<RenderedTemplate, Error> {
        let template = self.templates.get(template_dir).ok_or(Error::NotFound)?;

        let glob = format!("{}/**/*", template_dir.display());
        let mut tera = Tera::new();
        tera.load_from_glob(&glob)?;
        tera.autoescape_on(super::AUTOESCAPE_SUFFIXES);

        let entry_name = template
            .entry_path
            .strip_prefix(template_dir)
            .map_err(|_| Error::InvalidEntry)?
            .to_string_lossy()
            .into_owned();

        let context = tera::Context::from_serialize(cv_data)?;

        Ok(RenderedTemplate::new(tera.render(&entry_name, &context)?))
    }
}
