#[cfg(feature = "aur")]
mod aur;
#[cfg(feature = "github")]
mod github;
#[cfg(feature = "gitlab")]
mod gitlab;

// this is what `get_latest`s return
#[derive(Debug)]
pub struct Release {
    pub name: String,
    pub tag: Option<String>,
    pub url: String,
}

pub struct ApiArgs {
    pub request_client: reqwest::Client,
    pub package: String,
    //pub target: String, // equivalent to ex. `github = "adamperkowski/nvrs"` in the config
    //pub host: Option<String>,
    pub args: Vec<String>,
    pub api_key: Option<String>,
}

// this is necessary because we need to store a reference to an async function in `Api`
type ReleaseFuture =
    std::pin::Pin<Box<dyn std::future::Future<Output = crate::error::Result<Release>> + Send>>;

pub struct Api {
    pub name: &'static str,
    pub func: fn(ApiArgs) -> ReleaseFuture,
}

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
        StatusCode::NOT_FOUND => Err(error::Error::RequestNotFound),
        _ => Err(error::Error::RequestNotOK),
    }
}

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

// TODO: tests
