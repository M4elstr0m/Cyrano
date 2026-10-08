use crate::{crypto, cv_data::CvData};

use super::{Error, UnlockedProfile};
use std::path::PathBuf;

pub struct LockedProfile {
    // Identity
    name: String,
    /// Directory containing a profile (data files & assets folder)
    path: PathBuf,
}

impl LockedProfile {
    pub fn new(name: String, path: PathBuf) -> Self {
        Self { name, path }
    }

    pub fn unlock(&self, password: &str) -> Result<UnlockedProfile, Error> {
        let blob = std::fs::read(super::data_path(&self.path))?;

        if blob.len() < crypto::SALT_LEN {
            return Err(Error::CorruptedProfileFile);
        }

        let (salt_bytes, rest) = blob.split_at(crypto::SALT_LEN);
        let salt: crypto::Salt = salt_bytes
            .try_into()
            .map_err(|_| Error::CorruptedProfileFile)?;

        let key = crypto::derive_key(password, &salt)?;

        let plaintext = crypto::decrypt_with_key(&key, rest)?;
        let data = CvData::from_bytes(&plaintext)?;

        UnlockedProfile::new(self.name.clone(), self.path.clone(), data, salt, key)
    }
}
