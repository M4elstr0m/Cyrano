use super::{Error, Identity, LockedProfile, UnlockedProfile};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub struct Store {
    locked: HashMap<PathBuf, LockedProfile>,
    unlocked: HashMap<PathBuf, UnlockedProfile>,
}

impl Store {
    pub fn load(root_profiles_dir: &Path) -> Result<(Self, Vec<(PathBuf, Error)>), Error> {
        std::fs::create_dir_all(root_profiles_dir)?;

        let mut locked: HashMap<PathBuf, LockedProfile> = HashMap::new();

        let mut identity_warnings: Vec<(PathBuf, Error)> = vec![];

        for entry in std::fs::read_dir(root_profiles_dir)? {
            let profile_dir = entry?.path();

            if !profile_dir.is_dir() {
                continue;
            }

            match Identity::load(&profile_dir) {
                Ok(identity) => {
                    locked.insert(profile_dir, LockedProfile::new(identity));
                }
                Err(Error::NoIdentity) => {
                    // just check for data to decide whether warn or not
                    if super::data_path(&profile_dir).exists() {
                        identity_warnings.push((profile_dir, Error::NoIdentity));
                    }
                    continue;
                }
                Err(e) => identity_warnings.push((profile_dir, e)),
            }
        }

        Ok((
            Store {
                locked,
                unlocked: HashMap::new(),
            },
            identity_warnings,
        ))
    }

    pub fn locked(&self, profile_dir: &Path) -> Option<&LockedProfile> {
        self.locked.get(profile_dir)
    }

    pub fn locked_mut(&mut self, profile_dir: &Path) -> Option<&mut LockedProfile> {
        self.locked.get_mut(profile_dir)
    }

    pub fn unlocked(&self, profile_dir: &Path) -> Option<&UnlockedProfile> {
        self.unlocked.get(profile_dir)
    }

    pub fn unlocked_mut(&mut self, profile_dir: &Path) -> Option<&mut UnlockedProfile> {
        self.unlocked.get_mut(profile_dir)
    }

    pub fn lock_profile(&mut self, profile_dir: &Path) -> Result<(), Error> {
        let unlocked = self.unlocked.remove(profile_dir).ok_or(Error::NotFound)?;

        self.locked
            .insert(profile_dir.to_path_buf(), unlocked.lock());

        Ok(())
    }

    pub fn unlock_profile(&mut self, profile_dir: &Path, password: &str) -> Result<(), Error> {
        let locked = self.locked.get(profile_dir).ok_or(Error::NotFound)?;

        let unlocked = locked.unlock(profile_dir, password)?;

        self.locked.remove(profile_dir);
        self.unlocked.insert(profile_dir.to_path_buf(), unlocked);

        Ok(())
    }
}
