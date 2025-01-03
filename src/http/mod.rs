pub use api_url::{PackageQueryOptions, MajorVersionsQueryOptions};
use reqwest::blocking::Client;
use std::time::Duration;

use crate::{major_versions::MajorVersionList, package::PackageList, Error};

mod api_url;

const API_DEFAULT_URL: &str = "https://api.foojay.io/disco/";
const CONNECT_TIMEOUT_MS: u32 = 3000;

pub fn pull_packages(
    api_url: Option<impl std::fmt::Display>,
    query_opts: Option<PackageQueryOptions>,
) -> Result<PackageList, Error> {
    let client = new_client().map_err(Error::Http)?;

    let r = client
        .get(api_url::create_package_query_url(api_url.map(|u| u.to_string()).unwrap_or(API_DEFAULT_URL.to_string()), query_opts)?)
        .send()
        .map_err(Error::Http)?
        .error_for_status()
        .map_err(|e| Error::HttpResponse(e.to_string()))?;

    let raw_list = r.text().map_err(Error::Http)?;

    PackageList::decode(raw_list).map_err(Error::JsonParse)
}

pub fn pull_major_versions(
    api_url: Option<impl std::fmt::Display>,
    query_opts: Option<MajorVersionsQueryOptions>,
) -> Result<MajorVersionList, Error> {
    let client = new_client().map_err(Error::Http)?;

    let r = client
        .get(api_url::create_major_versions_query_url(api_url.map(|u| u.to_string()).unwrap_or(API_DEFAULT_URL.to_string()), query_opts)?)
        .send()
        .map_err(Error::Http)?
        .error_for_status()
        .map_err(|e| Error::HttpResponse(e.to_string()))?;

    let raw_list = r.text().map_err(Error::Http)?;

    MajorVersionList::decode(raw_list).map_err(Error::JsonParse)
}

pub fn new_client() -> Result<Client, reqwest::Error> {
    Client::builder()
        .connect_timeout(Duration::from_millis(CONNECT_TIMEOUT_MS as u64))
        .timeout(None)
        .build()
}
