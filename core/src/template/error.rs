#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Parse(#[from] toml::de::Error),

    #[error(transparent)]
    Serialize(#[from] toml::ser::Error),

    #[error("template folder does not contain a marker file")]
    NoMarker,

    // mostly because no valid name
    #[error("template's marker file is corrupted or malformed")]
    InvalidMarker,
}
