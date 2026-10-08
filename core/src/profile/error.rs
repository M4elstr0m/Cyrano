#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    CvData(#[from] crate::cv_data::Error),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error("profile data is corrupted or truncated")]
    InvalidData,

    #[error(transparent)]
    Crypto(#[from] crate::crypto::Error),
}
