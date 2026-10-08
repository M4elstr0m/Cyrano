mod locked;
use std::path::{Path, PathBuf};

pub use locked::*;

mod unlocked;
pub use unlocked::*;

mod error;
pub use error::Error;

mod store;

fn data_path(root: &Path) -> PathBuf {
    root.join("data.bin")
}

fn assets_dir(root: &Path) -> PathBuf {
    root.join("assets")
}
