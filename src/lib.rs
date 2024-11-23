// nvrs - fast new version checker for software releases 🦀

// thiserror implementation
pub mod error;

// communication with sources
pub mod api;

// command-line arguments
pub mod cli;

// operations on configuration files
pub mod config;

// operations on version files
pub mod verfiles;

// example "core" vars structure
pub struct Core {
    pub cli: cli::Cli,
    pub config: config::Config,
    pub verfiles: (verfiles::Verfile, verfiles::Verfile),
    pub client: reqwest::Client,
}

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
        Err(error::Error::SourceNotFound)
    }
}
