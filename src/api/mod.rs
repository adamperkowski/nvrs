#[cfg(feature = "aur")]
pub mod aur;
//#[cfg(feature = "github")]
//pub mod github;
//#[cfg(feature = "gitlab")]
//pub mod gitlab;

// this is what `get_latest`s return
#[derive(Debug)]
pub struct Release {
    pub name: String,
    pub tag: Option<String>,
    pub url: Option<String>,
}

pub struct ApiArgs {
    pub request_client: reqwest::Client,
    pub package: String,
    pub target: String, // equivalent to ex. `github = "adamperkowski/nvrs"` in the config
    pub host: Option<String>,
    pub api_key: Option<String>,
}

// this is necessary because we need to store a reference to an async function in Source
type ReleaseFuture = std::pin::Pin<Box<dyn std::future::Future<Output = Option<Release>> + Send>>;

// TODO: consider not using ReleaseFuture & Source. just calling by name

#[cfg(feature = "http")]
pub fn setup_headers() -> reqwest::header::HeaderMap {
    use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("nvrs"));

    headers
}

#[cfg(feature = "http")]
pub fn match_statuscode(req: &reqwest::Response) -> crate::error::Result<()> {
    use crate::error;
    use reqwest::StatusCode;

    let status = req.status();

    match status {
        StatusCode::OK => Ok(()),
        StatusCode::FORBIDDEN => Err(error::Error::RequestForbidden),
        _ => Err(error::Error::RequestNotOK),
    }
}

// TODO: tests
