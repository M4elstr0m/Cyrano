#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("blob is too short to contain a salt and nonce")]
    InvalidBlob,

    #[error(transparent)]
    Argon2(#[from] argon2::Error),

    #[error(transparent)]
    Random(#[from] getrandom::Error),

    #[error(transparent)]
    Chacha(#[from] chacha20poly1305::Error),
}
