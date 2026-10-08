use std::path::{Path, PathBuf};

mod locked;
pub use locked::LockedProfile;

mod unlocked;
pub use unlocked::UnlockedProfile;

mod error;
pub use error::Error;

mod store;
pub use store::ProfileStore as Store;

fn data_path(root: &Path) -> PathBuf {
    root.join("data.bin")
}

fn assets_dir(root: &Path) -> PathBuf {
    root.join("assets")
}
