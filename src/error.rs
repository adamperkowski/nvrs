use colored::Colorize;
use thiserror::Error as ThisError;

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

    #[error("request returned 430\nwe might be getting rate-limited here")]
    RequestForbidden,

    #[error("version not found")]
    NoVersion,

    #[error("specified config file not found")]
    NoConfigSpecified,

    #[error("no config found\nconfig file locations:\n ~/.config/nvrs.toml\n ./nvrs.toml\nmake sure the file is not empty")]
    NoConfig,
}

pub type Result<T> = std::result::Result<T, Error>;

/*pub fn print_error(error: Error, force: Option<bool>) {
    let error = error.to_string();
    let err = error.split_once(':').unwrap_or_default();
    custom_error(err.0, format!("\n{}", err.1), force);
}*/

pub fn custom_error(title: &'static str, message: String, exit: bool, force: bool) {
    println!("! {}{}", title.red(), message.replace("\n", "\n  "));

    if exit {
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Error as IOError;

    #[test]
    fn test_error() {
        let message = "nvrs died. now why could that be...?";
        let error = Error::from(IOError::other(message));
        assert_eq!(
            format!("\"io error: `{message}`\""),
            format!("{:?}", error.to_string())
        )
    }
}
