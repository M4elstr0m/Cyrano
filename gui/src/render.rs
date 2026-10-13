use crate::state::AppState;
use base64::{Engine, engine::general_purpose::STANDARD};
use cyrano_core::{cv_data::CvData, profile::Asset, template::RenderedTemplate};
use dioxus::prelude::ReadableExt;
use hayro::{
    PixmapSettings, RenderCache, RenderSettings, hayro_interpret::InterpreterSettings,
    hayro_syntax::Pdf, render, vello_cpu::color::palette::css::WHITE,
};
use std::path::PathBuf;

mod error;
pub use error::Error;

const PREVIEW_SCALE: f32 = 1.5;

pub struct Request {
    pub template_dir: PathBuf,
    pub profile_dir: PathBuf,
    pub cv: CvData,
}

fn render_template(state: AppState, request: &Request) -> Result<RenderedTemplate, Error> {
    let templates = state.templates.read();
    let template = templates
        .template(&request.template_dir)
        .ok_or(Error::TemplateMissing)?;

    let profiles = state.profiles.read();
    let profile = profiles
        .unlocked(&request.profile_dir)
        .ok_or(Error::ProfileLocked)?;

    let assets = template
        .required_assets
        .iter()
        .map(|asset| profile.load_asset(&request.profile_dir, asset))
        .collect::<Result<Vec<Asset>, _>>()?;

    Ok(template.render(&request.template_dir, &request.cv, &assets)?)
}

pub async fn preview(state: AppState, request: Request) -> Result<Vec<String>, Error> {
    let rendered = render_template(state, &request)?;

    let directory = std::env::temp_dir().join("cyrano_preview");
    let pdf = rendered.to_pdf(&directory).await?;
    let bytes = std::fs::read(&pdf);
    let _ = std::fs::remove_file(&pdf);

    tokio::task::spawn_blocking(move || rasterize(bytes?))
        .await
        .map_err(|error| Error::Raster(error.to_string()))?
}

pub async fn export(state: AppState, request: Request) -> Result<PathBuf, Error> {
    let rendered = render_template(state, &request)?;
    let directory = state.preferences.read().output_dir()?;
    let template_name = state
        .templates
        .read()
        .template(&request.template_dir)
        .map(|template| template.name.clone())
        .unwrap_or_default();

    let generated = rendered.to_pdf(&directory).await?;
    let target = directory.join(file_name(&request.cv.name, &template_name));
    std::fs::rename(&generated, &target)?;

    Ok(target)
}

fn file_name(person: &str, template: &str) -> String {
    let clean = |text: &str| {
        text.split_whitespace()
            .collect::<Vec<_>>()
            .join("_")
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '-' {
                    c
                } else {
                    '_'
                }
            })
            .collect::<String>()
    };

    let person = clean(person);
    let template = clean(template);

    match (person.is_empty(), template.is_empty()) {
        (true, true) => "CV.pdf".to_owned(),
        (false, true) => format!("{person}.pdf"),
        (true, false) => format!("CV-{template}.pdf"),
        (false, false) => format!("{person}-{template}.pdf"),
    }
}

fn rasterize(pdf: Vec<u8>) -> Result<Vec<String>, Error> {
    let pdf = Pdf::new(pdf).map_err(|error| Error::Raster(format!("{error:?}")))?;

    let cache = RenderCache::new();
    let interpreter = InterpreterSettings::default();
    let settings = RenderSettings::default();
    let pixmap = PixmapSettings {
        x_scale: PREVIEW_SCALE,
        y_scale: PREVIEW_SCALE,
        bg_color: WHITE,
    };

    pdf.pages()
        .iter()
        .map(|page| {
            let png = render(page, &cache, &interpreter, &settings, &pixmap)
                .into_png()
                .map_err(|error| Error::Raster(format!("{error:?}")))?;

            Ok(format!("data:image/png;base64,{}", STANDARD.encode(png)))
        })
        .collect()
}
