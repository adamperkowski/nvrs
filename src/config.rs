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
use tokio::{fs, io::AsyncWriteExt};

/// main configuration file structure
///
/// see the [example `nvrs.toml`](https://github.com/adamperkowski/nvrs/blob/main/nvrs.toml)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    /// `__config__` table
    pub __config__: Option<ConfigTable>,
    /// list of custom package tables
    #[serde(flatten)]
    pub packages: BTreeMap<String, Package>,
}

/// `__config__` table structure
///
/// see the [example `nvrs.toml`](https://github.com/adamperkowski/nvrs/blob/main/nvrs.toml)
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigTable {
    /// path to the `oldver` file
    pub oldver: Option<String>,
    /// path to the `newver` file
    pub newver: Option<String>,
    /// path to the keyfile
    pub(crate) keyfile: Option<String>,
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
    #[cfg(feature = "crates-io")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_empty_string")]
    cratesio: String,
    #[cfg(feature = "gitea")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_empty_string")]
    gitea: String,
    #[cfg(feature = "github")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_empty_string")]
    github: String,
    #[cfg(feature = "gitlab")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_empty_string")]
    gitlab: String,
    #[cfg(feature = "regex")]
    #[serde(default)]
    url: String,
    #[cfg(feature = "regex")]
    #[serde(default)]
    regex: String,
    #[cfg(feature = "shell")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_empty_string")]
    shell: String,

    /// whether to use the latest tag instead of the latest release
    #[serde(default)]
    pub(crate) use_max_tag: Option<bool>,
    /// prefix to add to the version name
    #[serde(default)]
    #[serde(skip_serializing_if = "is_empty_string")]
    pub prefix: String,
}

impl Package {
    /// manually create a new package entry
    pub fn new(
        source: String,
        target: String,
        use_max_tag: bool,
        prefix: String,
    ) -> error::Result<Self> {
        let mut package = Package::default();

        match source.as_str() {
            #[cfg(feature = "aur")]
            "aur" => {
                package.aur = target;
                Ok(())
            }
            #[cfg(feature = "crates-io")]
            "cratesio" => {
                package.cratesio = target;
                Ok(())
            }
            #[cfg(feature = "gitea")]
            "gitea" => {
                package.gitea = target;
                Ok(())
            }
            #[cfg(feature = "github")]
            "github" => {
                package.github = target;
                Ok(())
            }
            #[cfg(feature = "gitlab")]
            "gitlab" => {
                package.gitlab = target;
                Ok(())
            }
            #[cfg(feature = "regex")]
            "regex" => {
                package.url = target;
                Ok(())
            }
            #[cfg(feature = "shell")]
            "shell" => {
                package.shell = target;
                Ok(())
            }
            _ => Err(error::Error::SourceNotFound(source.clone())),
        }?;

        package.source = source;
        package.use_max_tag = Some(use_max_tag);
        package.prefix = prefix;

        Ok(package)
    }

    fn default() -> Self {
        Package {
            source: String::new(),
            host: String::new(),
            #[cfg(feature = "aur")]
            aur: String::new(),
            #[cfg(feature = "crates-io")]
            cratesio: String::new(),
            #[cfg(feature = "gitea")]
            gitea: String::new(),
            #[cfg(feature = "github")]
            github: String::new(),
            #[cfg(feature = "gitlab")]
            gitlab: String::new(),
            #[cfg(feature = "regex")]
            url: String::new(),
            #[cfg(feature = "regex")]
            regex: String::new(),
            #[cfg(feature = "shell")]
            shell: String::new(),
            use_max_tag: None,
            prefix: String::new(),
        }
    }

    /// global function to get various API-specific agrs for a package
    ///
    /// # example
    /// ```rust
    /// use nvrs::config::Package;
    ///
    /// let package = Package::new("github".to_string(), "adamperkowski/nvrs".to_string(),
    /// false, "v".to_string()).unwrap();
    ///
    /// let args = package.get_api();
    ///
    /// assert_eq!(args, ("github".to_string(), vec!["adamperkowski/nvrs".to_string()]))
    /// ```
    pub fn get_api(&self) -> (String, Vec<String>) {
        let self_ref = self.to_owned();
        let args = match self.source.as_str() {
            #[cfg(feature = "aur")]
            "aur" => vec![self_ref.aur],
            #[cfg(feature = "crates-io")]
            "cratesio" => vec![self_ref.cratesio],
            #[cfg(feature = "gitea")]
            "gitea" => vec![self_ref.gitea, self_ref.host],
            #[cfg(feature = "github")]
            "github" => vec![self_ref.github],
            #[cfg(feature = "gitlab")]
            "gitlab" => vec![self_ref.gitlab, self_ref.host],
            #[cfg(feature = "regex")]
            "regex" => vec![self_ref.url, self_ref.regex],
            #[cfg(feature = "shell")]
            "shell" => vec![self_ref.shell],
            _ => vec![],
        };

        (self_ref.source, args)
    }
}

/// global asynchronous function to load all config files
pub async fn load(custom_path: &Option<String>) -> error::Result<(Config, PathBuf)> {
    let config_path = if let Some(path) = custom_path {
        let path = Path::new(&path);
        if path.exists() && path.is_file() {
            path.to_path_buf()
        } else {
            return Err(error::Error::NoConfigSpecified);
        }
    } else {
        let default_path = Path::new("nvrs.toml");
        let config_home = format!(
            "{}/nvrs/nvrs.toml",
            env::var("XDG_CONFIG_HOME").unwrap_or(expand_tilde("~/.config".to_string())?)
        );
        let config_home_non_xdg = expand_tilde("~/.config/nvrs.toml".to_string())?;
        let config_home_non_xdg = Path::new(&config_home_non_xdg);
        let home_path = Path::new(&config_home);

        if default_path.exists() && default_path.is_file() {
            default_path.to_path_buf()
        } else if home_path.exists() && home_path.is_file() {
            home_path.to_path_buf()
        } else if config_home_non_xdg.exists() && config_home_non_xdg.is_file() {
            config_home_non_xdg.to_path_buf()
        } else {
            return Err(error::Error::NoConfig);
        }
    };

    let content = fs::read_to_string(&config_path).await?;
    let toml_content: Config = toml::from_str(&content)?;

    Ok((toml_content, config_path))
}

// FIXME: this nukes all the comments
/// global asynchronous function to save the config file
pub async fn save(config_content: &Config, path: PathBuf) -> error::Result<()> {
    let mut file = fs::File::create(path).await?;
    let content = format!("{}\n", toml::to_string(&config_content)?);
    file.write_all(content.as_bytes()).await?;
    file.shutdown().await?;

    Ok(())
}

pub(crate) fn expand_tilde(s: String) -> error::Result<String> {
    if let Some(stripped) = s.strip_prefix('~') {
        let home = env::var("HOME")?;
        return Ok(home + stripped);
    }
    Ok(s)
}

fn is_empty_string(s: &str) -> bool {
    s.is_empty()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn loading() {
        let config = load(&None).await.unwrap();

        assert_eq!(config.1, PathBuf::from("nvrs.toml"));
    }

    #[tokio::test]
    async fn manual_package() {
        assert!(
            Package::new(
                "non_existing_source".to_string(),
                "non_existing".to_string(),
                false,
                String::new()
            )
            .is_err()
        );
        assert!(
            Package::new(
                "github".to_string(),
                "orhun/git-cliff".to_string(),
                false,
                "v".to_string()
            )
            .is_ok()
        );
    }
}
