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

/// custom Error type for nvrs
#[derive(Debug, ThisError)]
pub enum Error {
    /// [reqwest] errors
    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),

    /// [std::io] errors
    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),

    /// [serde_json] errors
    #[error("json parsing error: {0}")]
    JSONError(#[from] serde_json::Error),

    /// [toml::de] errors
    #[error("toml parsing error: {0}")]
    TOMLError(#[from] toml::de::Error),

    /// [toml::ser] errors
    #[error("toml parsing error: {0}")]
    TOMLErrorSer(#[from] toml::ser::Error),

    // custom errors
    /// request status != OK
    #[error("{0}: request status != OK\n{1}")]
    RequestNotOK(String, String),

    /// request status == 430
    #[error("{0}: request returned 430\n{RATE_LIMIT}")]
    RequestForbidden(String),

    /// latest version of a package not found
    #[error("{0}: version not found")]
    NoVersion(String),

    /// specified configuration file not found
    #[error("specified config file not found")]
    NoConfigSpecified,

    /// configuration file not found in any of the default locations
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

    /// unsupported verfile version
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

/// custom Result type for nvrs
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
