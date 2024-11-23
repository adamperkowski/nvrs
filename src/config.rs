use crate::error;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    env,
    path::{Path, PathBuf},
};
use tokio::fs;

// keyfile structure
/*#[derive(Clone, Deserialize)]
struct KeysTable {
    #[cfg(feature = "github")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_empty_string")]
    github: String,
    #[cfg(feature = "gitlab")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_empty_string")]
    gitlab: String,
}
#[derive(Clone, Deserialize)]
struct Keyfile {
    keys: KeysTable,
}*/

// main config file
// __config__ structure
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigTable {
    pub oldver: Option<String>, // TODO: exceptions for empty oldver & newver entries
    pub newver: Option<String>,
    keyfile: Option<String>,
}

// package entry structure
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub __config__: Option<ConfigTable>,
    #[serde(flatten)]
    pub packages: BTreeMap<String, Package>,
}

impl Package {
    // global function to pass API agrs
    pub fn get_api(&self) -> (String, Option<Vec<String>>) {
        let args = match self.source.as_str() {
            #[cfg(feature = "aur")]
            "aur" => Some(vec![self.aur.clone()]),
            #[cfg(feature = "github")]
            "github" => Some(vec![self.github.clone()]),
            #[cfg(feature = "gitlab")]
            "gitlab" => Some(vec![self.gitlab.clone(), self.host.clone()]),
            _ => None,
        };

        (self.source.clone(), args)
    }
}

pub async fn load(
    custom_path: Option<String>,
) -> error::Result<(Config, PathBuf /*, Option<Keyfile>*/)> {
    if let Some(path) = custom_path {
        let config_path = Path::new(&path);
        if config_path.exists() && config_path.is_file() {
            let content = fs::read_to_string(config_path).await?;
            let toml_content: Config = toml::from_str(&content)?;

            return Ok((
                toml_content,
                PathBuf::from(config_path), /*, load_keyfile(toml_content)*/
            ));
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

    let toml_content = toml::from_str(&content)?;

    Ok((
        toml_content,
        path_final, /*, load_keyfile(toml_content)*/
    ))
}

fn is_empty_string(s: &str) -> bool {
    s.is_empty()
}

// TODO: tests
