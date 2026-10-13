#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Preferences(#[from] crate::preferences::Error),

    #[error(transparent)]
    Profile(#[from] cyrano_core::profile::Error),

    #[error(transparent)]
    Template(#[from] cyrano_core::template::Error),
}
