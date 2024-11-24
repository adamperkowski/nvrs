//! operations on keyfiles
//!
//! see the [example `nvrs.toml`](https://github.com/adamperkowski/nvrs/blob/main/nvrs.toml) & [example `keyfile.toml`](https://github.com/adamperkowski/nvrs/blob/main/n_keyfile.toml)

use crate::{config, error};
use serde::Deserialize;
use std::path::Path;
use tokio::fs;

/// keyfile structure
///
/// see `keyfile` in [crate::config::ConfigTable]
#[derive(Clone, Deserialize)]
pub struct Keyfile {
    keys: KeysTable,
}

/// `[keys]` table structure
///
/// see the [example `keyfile.toml`](https://github.com/adamperkowski/nvrs/blob/main/n_keyfile.toml)
#[derive(Clone, Deserialize)]
struct KeysTable {
    #[cfg(feature = "github")]
    #[serde(default)]
    #[serde(skip_serializing_if = "config::is_empty_string")]
    github: String,
    #[cfg(feature = "gitlab")]
    #[serde(default)]
    #[serde(skip_serializing_if = "config::is_empty_string")]
    gitlab: String,
}

impl Keyfile {
    /// returns API key for the specified API name (empty string if not found)
    pub async fn get_key(&self, api_name: &str) -> String {
        match api_name {
            #[cfg(feature = "github")]
            "github" => self.keys.github.clone(),
            #[cfg(feature = "gitlab")]
            "gitlab" => self.keys.gitlab.clone(),
            _ => String::new(),
        }
    }
}

/// load contents of the specified keyfile
///
/// see `keyfile` in [crate::config::ConfigTable]
pub async fn load(config_content: Option<config::ConfigTable>) -> error::Result<Option<Keyfile>> {
    if let Some(config_table) = config_content {
        if let Some(keyfile) = config_table.keyfile {
            let keyfile_path = Path::new(&keyfile);
            let keyfile_content = if keyfile_path.exists() && keyfile_path.is_file() {
                fs::read_to_string(keyfile_path).await?
            } else {
                String::new()
            };

            if keyfile_content.is_empty() {
                return Err(error::Error::NoKeyfile);
            }

            Ok(Some(toml::from_str(&keyfile_content)?))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}
