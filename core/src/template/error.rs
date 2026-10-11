#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Parse(#[from] toml::de::Error),

    #[error(transparent)]
    Serialize(#[from] toml::ser::Error),

    #[error(transparent)]
    Tera(#[from] tera::Error),

    #[error(transparent)]
    Mestra(#[from] mestra_core::Error),

    #[error("template folder does not contain a marker file")]
    NoMarker,

    // mostly because no valid name
    #[error("template's marker file is corrupted or malformed")]
    InvalidMarker,

    #[error("template's marker file has no valid template entry point")]
    NoEntry,

    // mostly security issues
    #[error("template entry file is corrupted or malformed")]
    InvalidEntry,

    #[error("no template was found at this location")]
    NotFound,
}
