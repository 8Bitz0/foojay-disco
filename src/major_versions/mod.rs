use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MajorVersionList {
    pub result: Vec<MajorVersion>,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MajorVersion {
    pub major_version: u32,
    pub term_of_support: String,
    pub maintained: bool,
    pub early_access_only: bool,
    pub release_status: String,
    pub versions: Vec<String>,
}

impl MajorVersionList {
    pub fn decode(json: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&json)
    }
}
