//! operations on configuration files
//!
//! see the [example `nvrs.toml`](https://github.com/adamperkowski/nvrs/blob/main/nvrs.toml)

use crate::error;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    env,
    path::{Path, PathBuf},
};
use tokio::fs;

/// main configuration file structure
///
/// see the [example `nvrs.toml`](https://github.com/adamperkowski/nvrs/blob/main/nvrs.toml)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub __config__: Option<ConfigTable>,
    #[serde(flatten)]
    pub packages: BTreeMap<String, Package>,
}

/// `__config__` table structure
///
/// see the [example `nvrs.toml`](https://github.com/adamperkowski/nvrs/blob/main/nvrs.toml)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigTable {
    pub oldver: Option<String>,
    pub newver: Option<String>,
    pub keyfile: Option<String>,
}

/// package entry structure
///
/// see the [example `nvrs.toml`](https://github.com/adamperkowski/nvrs/blob/main/nvrs.toml)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Package {
    source: String, // ex. "github", "aur"
    #[serde(default)]
    #[serde(skip_serializing_if = "is_empty_string")]
    host: String, // ex. "gitlab.archlinux.org"

    // equivalent to `target` in api::ApiArgs
    #[cfg(feature = "aur")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_empty_string")]
    aur: String,
    #[cfg(feature = "github")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_empty_string")]
    github: String,
    #[cfg(feature = "gitlab")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_empty_string")]
    gitlab: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "is_empty_string")]
    pub prefix: String,
}

// TODO: Package defaults & tests
impl Package {
    /// global function to get various API-specific agrs for a package
    ///
    /// # example
    /// ```rust,ignore
    /// // package has `source = "github"` * `github = "adamperkowski/nvrs"` specified
    /// let args = package.get_api();
    ///
    /// assert_eq!(package, ("github", vec!["adamperkowski/nvrs"]))
    /// ```
    pub fn get_api(&self) -> (String, Vec<String>) {
        let args = match self.source.as_str() {
            #[cfg(feature = "aur")]
            "aur" => vec![self.aur.clone()],
            #[cfg(feature = "github")]
            "github" => vec![self.github.clone()],
            #[cfg(feature = "gitlab")]
            "gitlab" => vec![self.gitlab.clone(), self.host.clone()],
            _ => vec![],
        };

        (self.source.clone(), args)
    }
}

/// global asynchronous function to load all config files
pub async fn load(custom_path: Option<String>) -> error::Result<(Config, PathBuf)> {
    if let Some(path) = custom_path {
        let config_path = Path::new(&path);
        if config_path.exists() && config_path.is_file() {
            let content = fs::read_to_string(config_path).await?;
            let toml_content: Config = toml::from_str(&content)?;

            return Ok((toml_content, PathBuf::from(config_path)));
        } else {
            return Err(error::Error::NoConfigSpecified);
        }
    }

    let config_path = Path::new("nvrs.toml");
    let config_home = format!(
        "{}/nvrs.toml",
        env::var("XDG_CONFIG_HOME").unwrap_or_else(|_| {
            format!(
                "{}/.config",
                env::var("HOME").unwrap_or_else(|_| ".".to_string())
            )
        })
    );
    let config_home_path = Path::new(&config_home);

    let (content, path_final) = if config_path.exists() && config_path.is_file() {
        (
            fs::read_to_string(config_path).await?,
            PathBuf::from(config_path),
        )
    } else if config_home_path.exists() && config_home_path.is_file() {
        (
            fs::read_to_string(config_home_path).await?,
            PathBuf::from(config_home_path),
        )
    } else {
        (String::new(), PathBuf::new())
    };

    if content.is_empty() {
        return Err(error::Error::NoConfig);
    }

    Ok((toml::from_str(&content)?, path_final))
}

fn is_empty_string(s: &str) -> bool {
    s.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn loading() {
        let config = load(None).await.unwrap();

        // TODO: here, ref L47

        assert_eq!(config.1, PathBuf::from("nvrs.toml"));
    }
}
