// Configuration manager.

use anyhow::{anyhow, Result};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use std::path::{Path, PathBuf};
use toml::de::Error;

pub const DEFAULT_FILENAME: &'static str = ".age.toml";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub current_version: Version,
    pub files: Vec<FileConfig>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FileConfig {
    pub path: PathBuf,
    pub search: String,
    pub replace: String,
}

pub fn resolve_config(root: &PathBuf) -> Result<Config> {
    let config_path = root.join(Path::new(DEFAULT_FILENAME));
    if !config_path.exists() {
        return Err(anyhow!("Configuratio file is not exists."));
    }
    let config_text = read_to_string(config_path);
    if config_text.is_err() {
        return Err(anyhow!(config_text.unwrap_err()));
    }
    let config_data: Result<Config, Error> = toml::from_str(config_text.unwrap().as_ref());
    match config_data {
        Ok(data) => Ok(data),
        Err(err) => Err(anyhow!(err)),
    }
}
