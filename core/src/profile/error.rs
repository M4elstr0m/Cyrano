#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Parse(#[from] toml::de::Error),

    #[error(transparent)]
    Serialize(#[from] toml::ser::Error),

    #[error(transparent)]
    Crypto(#[from] crate::crypto::Error),

    #[error(transparent)]
    CvData(#[from] crate::cv_data::Error),

    #[error("profile data is corrupted or truncated")]
    CorruptedProfileFile,

    #[error("profile folder does not contain an identity marker file")]
    NoIdentity,

    // mostly because no valid name
    #[error("profile's identity is corrupted or malformed")]
    InvalidIdentity,

    #[error("no profile was found at this location")]
    NotFound,
}
