use crate::{crypto, cv_data::CvData};

use super::{Error, Identity, UnlockedProfile};
use std::path::Path;

pub struct LockedProfile {
    // Identity
    identity: Identity,
}

impl LockedProfile {
    pub fn new(identity: Identity) -> Self {
        Self { identity }
    }

    pub(super) fn unlock(
        &self,
        profile_dir: &Path,
        password: &str,
    ) -> Result<UnlockedProfile, Error> {
        let blob = std::fs::read(super::data_path(profile_dir))?;

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

        UnlockedProfile::new(self.identity.clone(), data, salt, key)
    }
}
