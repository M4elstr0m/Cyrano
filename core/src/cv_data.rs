use serde::{Deserialize, Serialize};

mod error;
pub use error::Error;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CvData {
    pub version: u32,

    pub name: String,
    pub title: String,
    pub description: String,

    pub age_display: Option<String>,
    pub location: Option<String>,
    pub email: Option<String>,
    pub links: Vec<Link>,

    pub projects: Vec<Project>,
    pub experience: Vec<Experience>,
    pub education: Vec<Education>,
    pub hard_skills: Vec<SkillGroup>,
    pub soft_skills: Vec<String>,
    pub languages: Vec<LanguageSkill>,
    pub hobbies: Vec<String>,
}

impl CvData {
    pub const MODEL_VERSION: u32 = 1;

    pub fn new(name: String) -> Self {
        Self {
            version: Self::MODEL_VERSION,
            name,
            title: None,
            description: None,
            age_display: None,
            location: None,
            email: None,
            links: Vec::new(),
            projects: Vec::new(),
            experience: Vec::new(),
            education: Vec::new(),
            hard_skills: Vec::new(),
            soft_skills: Vec::new(),
            languages: Vec::new(),
            hobbies: Vec::new(),
        }
    }

    pub(crate) fn content_hash(&self) -> Result<blake3::Hash, Error> {
        Ok(blake3::hash(&serde_json::to_vec(self)?))
    }

    pub(crate) fn to_bytes(&self) -> Result<Vec<u8>, Error> {
        Ok(serde_json::to_vec(self)?)
    }

    pub(crate) fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        Ok(serde_json::from_slice(bytes)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    pub label: String,
    pub url: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
    pub start: YearMonth,
    pub end: Option<YearMonth>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct YearMonth {
    pub year: u16,
    pub month: Option<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub title: String,
    pub date: DateRange,
    pub description: String,
    pub used_skills: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Experience {
    pub title: String,
    pub company: String,
    pub date: DateRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Education {
    pub title: String,
    pub date: DateRange,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillGroup {
    pub category: String,
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageSkill {
    pub name: String,
    pub level: String,
}
