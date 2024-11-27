//! operations on version files
//!
//! see `newver` & `oldver` in [crate::config::ConfigTable]

use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path};
use tokio::{fs, io::AsyncWriteExt};

use crate::{config, error};

// verfiles get created from this
const TEMPLATE: &str = r#"{
  "version": 2,
  "data": {}
}
"#;

/// main data structure
#[derive(Serialize, Deserialize)]
pub struct VerData {
    pub data: BTreeMap<String, VerPackage>,
}

/// package entry structure
#[derive(Clone, Serialize, Deserialize)]
pub struct VerPackage {
    pub version: String,
    #[serde(default)]
    pub gitref: String,
    #[serde(default)]
    pub url: String,
}

/// file structure
#[derive(Serialize, Deserialize)]
pub struct Verfile {
    version: u8,
    #[serde(flatten)]
    pub data: VerData,
}

/// load the verfiles specified in [crate::config::ConfigTable]
pub async fn load(config_table: Option<config::ConfigTable>) -> error::Result<(Verfile, Verfile)> {
    if config_table.is_none() {
        return Err(error::Error::NoConfigTable);
    }
    let config_table = config_table.unwrap();

    if config_table.oldver.is_some() && config_table.newver.is_some() {
        let oldver = load_file(Path::new(config_table.oldver.as_ref().unwrap())).await?;
        let newver = load_file(Path::new(config_table.newver.as_ref().unwrap())).await?;

        if oldver.version != 2 || newver.version != 2 {
            return Err(error::Error::VerfileVer);
        }

        Ok((oldver, newver))
    } else {
        Err(error::Error::NoXVer)
    }
}

/// save changes to the verfiles
pub async fn save(
    verfile: Verfile,
    is_oldver: bool,
    config_table: Option<config::ConfigTable>,
) -> error::Result<()> {
    let config_table = config_table.unwrap();
    let path = if is_oldver {
        Path::new(config_table.oldver.as_ref().unwrap())
    } else {
        Path::new(config_table.newver.as_ref().unwrap())
    };

    let mut file = fs::File::create(path).await?;
    let content = format!("{}\n", serde_json::to_string_pretty(&verfile)?);

    Ok(file.write_all(content.as_bytes()).await?)
}

async fn load_file(path: &Path) -> error::Result<Verfile> {
    if !path.exists() {
        let mut file = fs::File::create(path).await?;
        file.write_all(TEMPLATE.as_bytes()).await?;
    }
    let content = fs::read_to_string(path).await?;

    Ok(serde_json::from_str(&content)?)
}

// TODO: tests
