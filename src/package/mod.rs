use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct PackageList {
    pub result: Vec<Package>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Package {
    pub id: String,
    pub archive_type: String,
    pub distribution: String,
    pub major_version: u32,
    pub java_version: String,
    pub distribution_version: String,
    pub jdk_version: u32,
    pub latest_build_available: bool,
    pub release_status: String,
    pub term_of_support: String,
    pub operating_system: String,
    pub lib_c_type: String,
    pub architecture: String,
    pub fpu: String,
    pub package_type: String,
    pub javafx_bundled: bool,
    pub directly_downloadable: bool,
    pub filename: String,
    pub links: HashMap<String, String>,
    pub free_use_in_production: bool,
    pub tck_tested: String,
    pub tck_cert_uri: String,
    pub aqavit_certified: String,
    pub aqavit_cert_uri: String,
    pub size: i64,
    pub feature: Vec<Feature>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Feature {
    pub name: String,
    pub ui_string: String,
    pub api_string: String,
}

impl PackageList {
    pub fn decode(json: String) -> Result<Self, serde_json::Error> {
        serde_json::from_str(&json)
    }
}
