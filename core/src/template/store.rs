use super::{Error, Template};
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
}
