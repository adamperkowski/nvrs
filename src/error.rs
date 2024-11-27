//! [thiserror] implementation

use thiserror::Error as ThisError;

const RATE_LIMIT: &str = "we might be getting rate-limited here";
const CONFIG_PATHS: &str = "config file locations:
 ~/.config/nvrs.toml
 ./nvrs.toml";
const NOT_EMPTY: &str = "make sure the file is not empty";
const EXAMPLE_CONFIG_TABLE: &str = "example:
[__config__]
oldver = \"oldver.json\"
newver = \"newver.json\"";

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("json parsing error: {0}")]
    JSONError(#[from] serde_json::Error),

    #[error("toml parsing error: {0}")]
    TOMLError(#[from] toml::de::Error),

    #[error("toml parsing error: {0}")]
    TOMLErrorSer(#[from] toml::ser::Error),

    // custom errors
    #[error("{0}: request status != OK\n{1}")]
    RequestNotOK(String, String),

    #[error("{0}: request returned 430\n{RATE_LIMIT}")]
    RequestForbidden(String),

    #[error("{0}: version not found")]
    NoVersion(String),

    /// explicitly specified configuration file not found
    #[error("specified config file not found")]
    NoConfigSpecified,

    /// configuration file not found
    #[error("no config found\n{CONFIG_PATHS}\n{NOT_EMPTY}")]
    NoConfig,

    /// no `__config__` in the configuration file
    #[error("__config__ not specified\n{EXAMPLE_CONFIG_TABLE}")]
    NoConfigTable,

    /// keyfile specified in the configuration not found
    #[error("specified keyfile not found\n{NOT_EMPTY}")]
    NoKeyfile,

    /// no `oldver` or `newver` in `__config__`
    #[error("oldver & newver not specified\n{EXAMPLE_CONFIG_TABLE}")]
    NoXVer,

    /// verfile version != 2
    #[error("unsupported verfile version\nplease update your verfiles")]
    VerfileVer,

    /// package not found in newver
    #[error("{0}: package not in newver")]
    PkgNotInNewver(String),

    /// package not found in config
    #[error("{0}: package not in config")]
    PkgNotInConfig(String),

    /// source / API not found
    #[error("source {0} not found")]
    SourceNotFound(String),
}

pub type Result<T> = std::result::Result<T, Error>;

#[test]
fn test_error() {
    let message = "nvrs died. now why could that be...?";
    let error = Error::from(std::io::Error::other(message));
    assert_eq!(
        format!("\"io error: {message}\""),
        format!("{:?}", error.to_string())
    )
}
