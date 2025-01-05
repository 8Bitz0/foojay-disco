use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DistributionList {
    pub result: Vec<Distribution>,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DistributionInfo {
    pub result: [Distribution; 1],
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Distribution {
    pub name: String,
    pub api_parameter: String,
    pub maintained: bool,
    pub available: bool,
    pub build_of_openjdk: bool,
    pub build_of_graalvm: bool,
    pub official_uri: String,
    pub versions: Vec<String>,
}

impl DistributionList {
    pub fn decode(json: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&json)
    }
}

impl DistributionInfo {
    pub fn decode(json: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&json)
    }
}
