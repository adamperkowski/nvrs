//! this module handles management & communication with sources, also knows as APIs

#[cfg(feature = "aur")]
mod aur;
#[cfg(feature = "github")]
mod github;
#[cfg(feature = "gitlab")]
mod gitlab;
#[cfg(feature = "regex")]
mod regex;

/// struct containing the API name & a pointer to API's `get_latest` function
pub struct Api {
    /// name of the API
    pub name: &'static str,
    /// pointer to the API's `get_latest` function
    pub func: fn(ApiArgs) -> ReleaseFuture,
}

/// arguments passed to a source
pub struct ApiArgs {
    pub request_client: reqwest::Client,
    /// name of the package
    pub package: String,
    pub use_max_tag: Option<bool>,
    /// arguments passed to the source
    pub args: Vec<String>,
    pub api_key: String, // empty String if none
}

/// this is what `get_latest`s return
#[derive(Debug)]
pub struct Release {
    /// name of the package
    pub name: String,
    /// version of the package
    pub tag: Option<String>,
    /// url to the version's source
    pub url: String,
}

// this is necessary because we need to store a reference to an async function in `Api`
type ReleaseFuture =
    std::pin::Pin<Box<dyn std::future::Future<Output = crate::error::Result<Release>> + Send>>;

fn setup_headers() -> reqwest::header::HeaderMap {
    use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("nvrs"));

    headers
}

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
    #[cfg(feature = "regex")]
    Api {
        name: "regex",
        func: regex::get_latest,
    },
];

#[test]
fn statuscode_matching_test() {
    use reqwest::StatusCode;

    assert!(match_statuscode(&StatusCode::OK, String::new()).is_ok());
    assert!(match_statuscode(&StatusCode::IM_A_TEAPOT, String::new()).is_err());
}
