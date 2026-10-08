use std::path::{Path, PathBuf};

mod locked;
pub use locked::LockedProfile;

mod unlocked;
pub use unlocked::UnlockedProfile;

mod error;
pub use error::Error;

mod store;
pub use store::ProfileStore as Store;

fn data_path(profile_dir: &Path) -> PathBuf {
    profile_dir.join("data.bin")
}

fn assets_dir(profile_dir: &Path) -> PathBuf {
    profile_dir.join("assets")
}

fn identity_marker_path(profile_dir: &Path) -> PathBuf {
    profile_dir.join("IDENTITY.cyrano")
}
