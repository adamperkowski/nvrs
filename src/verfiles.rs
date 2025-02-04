//! operations on version files
//!
//! see `newver` & `oldver` in [crate::config::ConfigTable]

use crate::{config, error};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::Path};
use tokio::{fs, io::AsyncWriteExt};

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
    /// url pointing to the release
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
pub async fn load(config_table: &Option<config::ConfigTable>) -> error::Result<(Verfile, Verfile)> {
    let config_table = config_table.to_owned().ok_or(error::Error::NoConfigTable)?;

    let oldver_path = config::expand_tilde(config_table.oldver.ok_or(error::Error::NoXVer)?)?;
    let newver_path = config::expand_tilde(config_table.newver.ok_or(error::Error::NoXVer)?)?;

    let (oldver, newver) = tokio::try_join!(
        load_file(Path::new(&oldver_path)),
        load_file(Path::new(&newver_path))
    )?;

    if oldver.version != 2 || newver.version != 2 {
        return Err(error::Error::VerfileVer);
    }

    Ok((oldver, newver))
}

/// save changes to the verfiles
pub async fn save(
    verfile: &Verfile,
    is_oldver: bool,
    config_table: &Option<config::ConfigTable>,
) -> error::Result<()> {
    let config_table = config_table.to_owned().ok_or(error::Error::NoConfigTable)?;
    let path = if is_oldver {
        config_table.oldver.ok_or(error::Error::NoXVer)?
    } else {
        config_table.newver.ok_or(error::Error::NoXVer)?
    };

    let mut file = fs::File::create(Path::new(&config::expand_tilde(path)?)).await?;
    let content = format!("{}\n", serde_json::to_string_pretty(&verfile)?);

    file.write_all(content.as_bytes()).await?;
    file.shutdown().await?;

    Ok(())
}

async fn load_file(path: &Path) -> error::Result<Verfile> {
    if !path.exists() {
        let mut file = fs::File::create(path).await?;
        file.write_all(TEMPLATE.as_bytes()).await?;
        file.shutdown().await?;
    }

    let content = fs::read_to_string(path).await?;
    let verfile: Verfile = serde_json::from_str(&content)?;

    Ok(verfile)
}

// TODO: tests
