use super::Error;
use base64::{Engine, engine::general_purpose::STANDARD};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[derive(Debug, Clone)]
pub struct Asset {
    name: String,
    bytes: Vec<u8>,
}

impl Serialize for Asset {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.name)
    }
}

impl<'de> Deserialize<'de> for Asset {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Ok(Self::new(String::deserialize(deserializer)?, Vec::new()))
    }
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

    fn mime_type(&self) -> Option<&'static str> {
        if is_svg(&self.bytes) {
            Some("image/svg+xml")
        } else {
            infer::get(&self.bytes).map(|kind| kind.mime_type())
        }
    }

    pub fn data_uri(&self) -> Result<String, Error> {
        let mime = self.mime_type().ok_or(Error::UnrecognizedAssetFormat)?;

        Ok(format!(
            "data:{mime};base64,{}",
            STANDARD.encode(&self.bytes)
        ))
    }
}

fn is_svg(bytes: &[u8]) -> bool {
    let sample_len = bytes.len().min(512);
    String::from_utf8_lossy(&bytes[..sample_len])
        .to_ascii_lowercase()
        .contains("<svg")
}
