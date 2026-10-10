use zeroize::Zeroizing;

use super::{Error, Identity, LockedProfile};
use crate::crypto;
use crate::cv_data::CvData;
use std::path::Path;

pub struct UnlockedProfile {
    // Identity
    identity: Identity,

    // Data
    data: CvData,
    initial_data_hash: blake3::Hash,

    // Security
    salt: crypto::Salt,
    key: Zeroizing<crypto::DerivedKey>,
}

impl UnlockedProfile {
    pub(super) fn new(
        identity: Identity,
        data: CvData,
        salt: crypto::Salt,
        key: Zeroizing<crypto::DerivedKey>,
    ) -> Result<Self, Error> {
        let initial_data_hash = data.content_hash()?;

        Ok(UnlockedProfile {
            identity,
            data,
            initial_data_hash,
            salt,
            key,
        })
    }

    pub fn identity(&self) -> &Identity {
        &self.identity
    }

    pub fn data(&self) -> &CvData {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut CvData {
        &mut self.data
    }

    fn refresh_data_hash(&mut self) -> Result<(), Error> {
        self.initial_data_hash = self.data.content_hash()?;

        Ok(())
    }

    pub fn is_modified(&self) -> Result<bool, Error> {
        Ok(self.data.content_hash()? != self.initial_data_hash)
    }

    pub(super) fn lock(self) -> LockedProfile {
        LockedProfile::new(self.identity)
    }

    pub fn save(&mut self, profile_dir: &Path) -> Result<(), Error> {
        let plaintext = self.data.to_bytes()?;
        let key_blob = crypto::encrypt_with_key(&self.key, &plaintext)?;

        let mut blob = Vec::with_capacity(self.salt.len() + key_blob.len());
        blob.extend_from_slice(&self.salt);
        blob.extend_from_slice(&key_blob);

        std::fs::write(super::data_path(profile_dir), blob)?;

        self.refresh_data_hash()?;

        Ok(())
    }

    pub fn save_asset(
        &self,
        profile_dir: &Path,
        filename: &str,
        bytes: &[u8],
    ) -> Result<(), Error> {
        let assets_dir = super::assets_dir(profile_dir);

        std::fs::create_dir_all(&assets_dir)?;

        let blob = crypto::encrypt_with_key(&self.key, bytes)?;

        std::fs::write(assets_dir.join(filename), blob)?;

        Ok(())
    }

    pub fn load_asset(&self, profile_dir: &Path, filename: &str) -> Result<Vec<u8>, Error> {
        let blob = std::fs::read(super::assets_dir(profile_dir).join(filename))?;

        Ok(crypto::decrypt_with_key(&self.key, &blob)?)
    }
}
