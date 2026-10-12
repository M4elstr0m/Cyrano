use std::path::{Path, PathBuf};

use super::Error;
use mestra_core as mestra;

pub struct RenderedTemplate(String);

const TEMP_FILE_PREFIX: &'static str = "cyrano_template";

impl RenderedTemplate {
    pub(super) fn new(content: String) -> Self {
        Self(content)
    }

    pub fn clear_temp_files() -> Result<(), Error> {
        for entry in std::fs::read_dir(std::env::temp_dir())? {
            let path = entry?.path();

            let Some(name) = path.file_name().and_then(|n| n.to_str()) else {
                continue;
            };

            if name.starts_with(TEMP_FILE_PREFIX) {
                let _ = std::fs::remove_file(&path);
            }
        }

        Ok(())
    }

    fn write_to_html(&self) -> Result<PathBuf, Error> {
        static COUNTER: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
        let n = COUNTER.fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        let path = std::env::temp_dir().join(format!(
            "{TEMP_FILE_PREFIX}-{}_{n}.html",
            std::process::id()
        ));
        std::fs::write(&path, &self.0)?;

        Ok(path)
    }

    pub async fn to_pdf(&self, output_dir: &Path) -> Result<PathBuf, Error> {
        let html_path = self.write_to_html()?;

        let html_file =
            mestra::File::new_with_kind(mestra::Kind::Html, html_path.to_string_lossy());

        let output_pdf = html_file.try_convert_to(mestra::Kind::Pdf).await;

        let _ = std::fs::remove_file(&html_path);

        let output_path = PathBuf::from(output_pdf?.path());

        std::fs::create_dir_all(output_dir)?;

        let file_name = output_path
            .file_name()
            .expect("mestra always returns a path with a file name");
        let final_path = output_dir.join(file_name);

        std::fs::copy(&output_path, &final_path)?;
        let _ = std::fs::remove_file(&output_path);

        Ok(final_path)
    }
}
