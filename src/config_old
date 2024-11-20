use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    env, fs,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Deserialize)]
struct KeysTable {
    #[cfg(feature = "github")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_empty_string")]
    pub github: String,
    #[cfg(feature = "gitlab")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_empty_string")]
    pub gitlab: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Keyfile {
    keys: KeysTable,
}

impl Keyfile {
    pub fn get_api_key(&self, api_name: String) -> String {
        match api_name.as_str() {
            "github" => self.keys.github.clone(),
            "gitlab" => self.keys.gitlab.clone(),
            _ => String::new(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigTable {
    pub oldver: Option<String>,
    pub newver: Option<String>,
    keyfile: Option<String>,
    /* proxy: Option<String>,
    max_concurrency: Option<String>,
    http_timeout: Option<String>, */
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Package {
    pub source: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_empty_string")]
    pub host: String,

    #[cfg(feature = "aur")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_empty_string")]
    pub aur: String,
    #[cfg(feature = "github")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_empty_string")]
    pub github: String,
    #[cfg(feature = "gitlab")]
    #[serde(default)]
    #[serde(skip_serializing_if = "is_empty_string")]
    pub gitlab: String,

    #[serde(default)]
    #[serde(skip_serializing_if = "is_empty_string")]
    pub prefix: String,
}

impl Package {
    pub fn get_api_arg(&self, api_name: &str) -> Option<Vec<String>> {
        match api_name {
            #[cfg(feature = "aur")]
            "aur" => Some(vec![self.aur.clone()]),
            #[cfg(feature = "github")]
            "github" => Some(vec![self.github.clone()]),
            #[cfg(feature = "gitlab")]
            "gitlab" => Some(vec![self.gitlab.clone(), self.host.clone()]),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Config {
    pub __config__: Option<ConfigTable>,
    #[serde(flatten)]
    pub packages: BTreeMap<String, Package>,
}

pub fn load(custom_path: Option<String>) -> (Config, PathBuf, Option<Keyfile>) {
    if custom_path.is_some() {
        let custom_path = custom_path.unwrap();
        let config_path = Path::new(&custom_path);
        if config_path.exists() && config_path.is_file() {
            let content = fs::read_to_string(config_path).unwrap_or_default();
            let toml_content: Config =
                toml::from_str(&content).expect("failed to read the config file");

            return (
                toml_content.clone(),
                PathBuf::from(config_path),
                load_keyfile(toml_content),
            );
        } else {
            crate::custom_error("specified config file not found", String::new(), "");
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

    let (content, path_actual) = if config_path.exists() && config_path.is_file() {
        (
            fs::read_to_string(config_path).unwrap_or_default(),
            PathBuf::from(config_path),
        )
    } else if config_home_path.exists() && config_home_path.is_file() {
        (
            fs::read_to_string(config_home_path).unwrap_or_default(),
            PathBuf::from(config_home_path),
        )
    } else {
        (String::new(), PathBuf::new())
    };

    if content.is_empty() {
        crate::custom_error(
            "no config found",
            "\nconfig file locations:\n ~/.config/nvrs.toml\n ./nvrs.toml\nmake sure the file is not empty".to_string(),
         "");
    }

    let toml_content: Config = toml::from_str(&content).expect("error reading the config file");

    (
        toml_content.clone(),
        path_actual,
        load_keyfile(toml_content),
    )
}

fn load_keyfile(toml_content: Config) -> Option<Keyfile> {
    if let Some(config_content) = toml_content.__config__ {
        if let Some(keyfile) = config_content.keyfile {
            let keyfile_path = Path::new(&keyfile);
            let keyfile_content = if keyfile_path.exists() && keyfile_path.is_file() {
                fs::read_to_string(keyfile_path).unwrap_or_default()
            } else {
                String::new()
            };

            if keyfile_content.is_empty() {
                crate::custom_error(
                    "keyfile not found",
                    "\nmake sure the file is not empty".to_string(),
                    "exit",
                );
            }

            Some(toml::from_str(&keyfile_content).expect("error reading the keyfile"))
        } else {
            None
        }
    } else {
        None
    }
}

pub fn save(config_content: Config, path: PathBuf) -> Result<(), std::io::Error> {
    let mut file = fs::File::create(path).unwrap();
    let content = format!("{}\n", toml::to_string(&config_content).unwrap());
    file.write_all(content.as_bytes())
}

fn is_empty_string(value: &str) -> bool {
    value.is_empty()
}
