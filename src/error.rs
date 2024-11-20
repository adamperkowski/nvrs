use colored::Colorize;
use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[cfg(feature = "http")]
    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),

    #[error("io error: `{0}`")]
    IOError(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn print_error(error: Error, force: Option<bool>) {
    let error = error.to_string();
    let (error_t, error_m) = error.split_once(':').unwrap();
    custom_error(error_t, format!("\n{}", error_m), force);
}

pub fn custom_error(title: &'static str, message: String, force: Option<bool>) {
    println!("! {}{}", title.red(), message.replace("\n", "\n  "));
    if force.is_some_and(move |_| false) {
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
