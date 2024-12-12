//! nvrs - fast new version checker for software releases 🚦🦀
//!
//! <div class="warning">
//!
//! nvrs is still a WIP
//!
//! new features & bugfixes are being pushed every day
//!
//! you may encounter some issues. please consider [submitting feedback](https://github.com/adamperkowski/nvrs/issues/new/choose) if you do.
//!
//! </div>

pub mod api;
pub mod config;
pub mod error;
pub mod keyfile;
pub mod verfiles;

/// example "core" struct that holds all the necessary data for the library
///
/// # example usage
/// ```rust
/// # tokio_test::block_on(async {
/// use nvrs::*;
///
/// let config = config::load(&None).await.unwrap();
/// let verfiles = verfiles::load(&config.0.__config__).await.unwrap();
/// let keyfile = keyfile::load(&config.0.__config__).await.unwrap();
///
/// Core {
///     config,
///     verfiles,
///     client: reqwest::Client::new(),
///     keyfile,
/// };
/// # })
/// ```
pub struct Core {
    /// configuration file content and path
    pub config: (config::Config, std::path::PathBuf),
    /// verfiles contents
    pub verfiles: (verfiles::Verfile, verfiles::Verfile),
    /// reqwest HTTP client
    pub client: reqwest::Client,
    /// keyfile contents
    pub keyfile: Option<keyfile::Keyfile>,
}

/// an asynchronous function that package's source and gets the latest release
/// # example usage
/// ```rust
/// # tokio_test::block_on(async {
/// use nvrs::{run_source, config};
///
/// let package_name = "nvrs".to_string();
/// let package = config::Package::new("github".to_string(), "adamperkowski/nvrs".to_string(), false, "v".to_string()).unwrap();
///
/// let client = reqwest::Client::new();
///
/// run_source((package_name, package), client, None).await;
/// # })
/// ```
/// see [crate::config::Package] for `package`
pub async fn run_source(
    package: (String, config::Package),
    client: reqwest::Client,
    keyfile: Option<keyfile::Keyfile>,
) -> error::Result<api::Release> {
    let (source, api_args) = package.1.get_api();

    if let Some(api) = api::API_LIST.iter().find(|a| a.name == source) {
        let api_key = if let Some(keyfile_content) = keyfile {
            keyfile_content.get_key(api.name).await
        } else {
            String::new()
        };

        let args = api::ApiArgs {
            request_client: client,
            package: package.0,
            use_max_tag: package.1.use_max_tag,
            args: api_args,
            api_key,
        };

        Ok((api.func)(args).await?)
    } else {
        Err(error::Error::SourceNotFound(source.to_string()))
    }
}
