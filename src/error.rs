//! [thiserror] implementation

use colored::Colorize;
use thiserror::Error as ThisError;

const RATE_LIMIT: &str = "we might be getting rate-limited here";
const CONFIG_PATHS: &str = "config file locations:
 ~/.config/nvrs.toml
 ./nvrs.toml
make sure the file is not empty";
const EXAMPLE_CONFIG_TABLE: &str = "example:
[__config__]
oldver = \"oldver.json\"
newver = \"newver.json\"";

#[derive(Debug, ThisError)]
pub enum Error {
    #[cfg(feature = "http")]
    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),

    #[error("json parsing error: {0}")]
    JSONError(#[from] serde_json::Error),

    #[error("toml parsing error: {0}")]
    TOMLError(#[from] toml::de::Error),

    // custom errors
    #[error("request didn't return 200")]
    RequestNotOK,

    #[error("request returned 430\n{RATE_LIMIT}")]
    RequestForbidden,

    #[error("request returned 404 not found")]
    RequestNotFound,

    #[error("version not found")]
    NoVersion,

    #[error("specified config file not found")]
    NoConfigSpecified,

    #[error("no config found\n{CONFIG_PATHS}")]
    NoConfig,

    #[error("__config__ not specified\n{EXAMPLE_CONFIG_TABLE}")]
    NoConfigTable,

    #[error("oldver & newver not specified\n{EXAMPLE_CONFIG_TABLE}")]
    NoXVer,

    #[error("unsupported verfile version\nplease update your verfiles")]
    VerfileVer,

    #[error("package not in newver")]
    PkgNotInNewver,

    #[error("source not found")]
    SourceNotFound,
}

pub type Result<T> = std::result::Result<T, Error>;

/*
pub fn custom_error(title: &'static str, message: String, exit: bool /*, force: bool*/) {
    println!("! {}{}", title.red(), message.replace("\n", "\n  "));

    if exit {
        std::process::exit(1);
    }
}
*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Error as IOError;

    #[test]
    fn test_error() {
        let message = "nvrs died. now why could that be...?";
        let error = Error::from(IOError::other(message));
        assert_eq!(
            format!("\"io error: {message}\""),
            format!("{:?}", error.to_string())
        )
    }
}
