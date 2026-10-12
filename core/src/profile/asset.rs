use super::Error;
use base64::{Engine, engine::general_purpose::STANDARD};

pub struct Asset {
    name: String,
    bytes: Vec<u8>,
}

impl Asset {
    pub fn new(name: String, bytes: Vec<u8>) -> Self {
        Self { name, bytes }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn data_uri(&self) -> Result<String, Error> {
        let mime = infer::get(&self.bytes)
            .map(|kind| kind.mime_type())
            .ok_or(Error::UnrecognizedAssetFormat)?;

        Ok(format!(
            "data:{mime};base64,{}",
            STANDARD.encode(&self.bytes)
        ))
    }
}
