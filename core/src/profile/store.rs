use super::Error;
use std::path::Path;

pub struct ProfileStore {}

fn read_identity_marker(profile_dir: &Path) -> Result<String, Error> {
    let marker_content = match std::fs::read_to_string(super::identity_marker_path(profile_dir)) {
        Ok(v) => v,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Err(Error::NoIdentity),
        Err(e) => return Err(e.into()),
    };

    let name_field = marker_content
        .lines()
        .find_map(|line| line.strip_prefix("name="))
        .map(str::trim)
        .ok_or(Error::InvalidIdentity)?;

    Ok(name_field.to_owned())
}
