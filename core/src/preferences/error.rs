#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Parse(#[from] toml::de::Error),

    #[error(transparent)]
    Serialize(#[from] toml::ser::Error),

    #[error("could not determine the user's home directory")]
    NoHomeDirectory,

    #[error("could not find the parent directory")]
    NoParentDirectory,
}
