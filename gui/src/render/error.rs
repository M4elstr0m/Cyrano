#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Template(#[from] cyrano_core::template::Error),

    #[error(transparent)]
    Profile(#[from] cyrano_core::profile::Error),

    #[error(transparent)]
    Preferences(#[from] crate::preferences::Error),

    #[error("this template is no longer available")]
    TemplateMissing,

    #[error("this profile is locked")]
    ProfileLocked,

    #[error("the generated PDF could not be displayed: {0}")]
    Raster(String),
}
