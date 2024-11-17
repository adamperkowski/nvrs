use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    env, fs,
    io::Write,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfigTable {
    pub oldver: Option<String>,
    pub newver: Option<String>,
    /* proxy: Option<String>,
    max_concurrency: Option<String>,
    http_timeout: Option<String>,
    keyfile: Option<String>, */
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Package {
    pub source: String,
    #[serde(default)]
    pub host: String,

    #[cfg(feature = "aur")]
    #[serde(default)]
    pub aur: String,
    #[cfg(feature = "github")]
    #[serde(default)]
    pub github: String,
    #[cfg(feature = "gitlab")]
    #[serde(default)]
    pub gitlab: String,

    #[serde(default)]
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
    pub packages: HashMap<String, Package>,
}

pub fn load(custom_path: Option<String>) -> (Config, PathBuf) {
    if let Some(path) = custom_path {
        let config_path = Path::new(&path);
        if config_path.exists() && config_path.is_file() {
            let content = fs::read_to_string(config_path).unwrap_or_default();

            return (
                toml::from_str(&content).expect("failed to read the config file"),
                PathBuf::from(config_path),
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

    (
        toml::from_str(&content).expect("error reading the config file"),
        path_actual,
    )
}

pub fn save(config_content: Config, path: PathBuf) -> Result<(), std::io::Error> {
    let mut file = fs::File::create(path).unwrap();
    let content = format!("{}\n", toml::to_string(&config_content).unwrap());
    file.write_all(content.as_bytes())
}
