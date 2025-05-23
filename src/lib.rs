mod distributions;
mod http;
mod major_versions;
mod package;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("URL parse error: {0}")]
    UrlParse(url::ParseError),
    #[error("HTTP error: {0}")]
    Http(reqwest::Error),
    #[error("HTTP response error: {0}")]
    HttpResponse(String),
    #[error("JSON parse error: {0}")]
    JsonParse(serde_json::Error),
}

pub use http::{
    PackageQueryOptions,
    MajorVersionsQueryOptions,
    pull_distribution_info,
    pull_distributions,
    pull_major_versions,
    pull_packages,
    pull_package_info,
};
pub use distributions::{DistributionList, DistributionInfo, Distribution};
pub use major_versions::{MajorVersionList, MajorVersion};
pub use package::{PackageList, Package, Feature, PackageInfo, PackageInfoInner};
