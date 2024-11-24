//! this module handles management & communication with sources, also knows as APIs

#[cfg(feature = "aur")]
mod aur;
#[cfg(feature = "github")]
mod github;
#[cfg(feature = "gitlab")]
mod gitlab;

/// struct containing the API name & a pointer to API's `get_latest` function
pub struct Api {
    pub name: &'static str,
    pub func: fn(ApiArgs) -> ReleaseFuture,
}

/// arguments passed to a source
pub struct ApiArgs {
    pub request_client: reqwest::Client,
    pub package: String,
    pub args: Vec<String>,
    pub api_key: String, // empty String if none
}

/// this is what `get_latest`s return
#[derive(Debug)]
pub struct Release {
    pub name: String,
    pub tag: Option<String>,
    pub url: String,
}

// this is necessary because we need to store a reference to an async function in `Api`
type ReleaseFuture =
    std::pin::Pin<Box<dyn std::future::Future<Output = crate::error::Result<Release>> + Send>>;

#[cfg(feature = "http")]
fn setup_headers() -> reqwest::header::HeaderMap {
    use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("nvrs"));

    headers
}

#[cfg(feature = "http")]
fn match_statuscode(status: &reqwest::StatusCode, package: String) -> crate::error::Result<()> {
    use crate::error;
    use reqwest::StatusCode;

    match status.to_owned() {
        StatusCode::OK => Ok(()),
        StatusCode::FORBIDDEN => Err(error::Error::RequestForbidden(package)),
        _ => Err(error::Error::RequestNotOK(package, status.to_string())),
    }
}

/// public list of available sources
pub const API_LIST: &[Api] = &[
    #[cfg(feature = "aur")]
    Api {
        name: "aur",
        func: aur::get_latest,
    },
    #[cfg(feature = "github")]
    Api {
        name: "github",
        func: github::get_latest,
    },
    #[cfg(feature = "gitlab")]
    Api {
        name: "gitlab",
        func: gitlab::get_latest,
    },
];

#[test]
fn statuscode_matching_test() {
    use reqwest::StatusCode;

    assert!(match_statuscode(&StatusCode::OK, String::new()).is_ok());
    assert!(match_statuscode(&StatusCode::IM_A_TEAPOT, String::new()).is_err());
}
