use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, io::Write, path::Path};

use crate::config::ConfigTable;

const TEMPLATE: &str = r#"{
  "version": 2,
  "data": {
  }
}
"#;

const CONFIG_NONE_M: &str = "__config__ not specified\nexample:";
const XVER_NONE_M: &str = "oldver & newver not specified\nexample:";
const CONFIG_NONE_E: &str = "\n[__config__]
oldver = \"oldver.json\"
newver = \"newver.json\"";

#[derive(Clone, Serialize, Deserialize)]
pub struct Package {
    pub version: String,
    pub gitref: String,
    pub url: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Data {
    pub data: HashMap<String, Package>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Verfile {
    version: u8,
    #[serde(flatten)]
    pub data: Data,
}

pub fn load(config_table: Option<ConfigTable>) -> Option<(Verfile, Verfile)> {
    if config_table.is_none() {
        crate::custom_error(CONFIG_NONE_M, CONFIG_NONE_E.to_string());
    }
    let config_table = config_table.unwrap();

    if config_table.oldver.is_some() && config_table.newver.is_some() {
        let oldver_path = Path::new(config_table.oldver.as_ref().unwrap());
        let newver_path = Path::new(config_table.newver.as_ref().unwrap());
        let oldver = load_file(oldver_path);
        let newver = load_file(newver_path);

        if oldver.version != 2 || newver.version != 2 {
            crate::custom_error(
                "unsupported verfile version",
                "\nplease update your verfiles".to_string(),
            );
        }

        Some((oldver, newver))
    } else {
        crate::custom_error(XVER_NONE_M, CONFIG_NONE_E.to_string());
        None
    }
}

pub fn save(
    verfile: Verfile,
    oldver: bool,
    config_table: Option<ConfigTable>,
) -> Result<(), std::io::Error> {
    let config_table = config_table.unwrap();
    let path = if oldver {
        Path::new(config_table.oldver.as_ref().unwrap())
    } else {
        Path::new(config_table.newver.as_ref().unwrap())
    };

    let mut file = fs::File::create(path).unwrap();
    let content = format!("{}\n", serde_json::to_string_pretty(&verfile).unwrap());
    file.write_all(content.as_bytes())
}

fn load_file(path: &Path) -> Verfile {
    if !path.exists() {
        let mut file = fs::File::create(path).unwrap();
        file.write_all(TEMPLATE.as_bytes()).unwrap();
    }
    let content = fs::read_to_string(path).unwrap();

    serde_json::from_str(&content).expect("failed to read oldver")
}
