//! nvrs - fast new version checker for software releases 🚦🦀
//!
//! nvrs is still a WIP<br>
//! new features & bugfixes are being pushed every day<br>
//! you may encounter some issues. please consider [submitting feedback](https://github.com/adamperkowski/nvrs/issues/new/choose) if you do.

pub mod api;
pub mod config;
pub mod error;
pub mod verfiles;

/// example "core" vars structure
pub struct Core {
    pub config: config::Config,
    pub verfiles: (verfiles::Verfile, verfiles::Verfile),
    pub client: reqwest::Client,
}

/// an asynchronous function that package's source and gets the latest release
/// # example usage
/// ```rust
/// let package = "nvrs";
/// let client = reqwest::Client::new();
/// run_source(package, client).await;
/// ```
pub async fn run_source(
    package: (String, config::Package),
    client: reqwest::Client,
) -> error::Result<api::Release> {
    let (source, api_args) = package.1.get_api();

    if let Some(api) = api::API_LIST.iter().find(|a| a.name == source) {
        let args = api::ApiArgs {
            request_client: client,
            package: package.0,
            args: api_args,
            api_key: None,
        };

        Ok((api.func)(args).await?)
    } else {
        Err(error::Error::SourceNotFound(source))
    }
}
