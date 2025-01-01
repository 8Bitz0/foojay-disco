mod http;
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

pub use http::{QueryOptions, pull};
pub use package::{PackageList, Package, Feature};
