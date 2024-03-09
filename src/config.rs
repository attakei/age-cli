// Configuration manager.

use semver::Version;
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs::read_to_string;
use std::path::{Path, PathBuf};

pub const DEFAULT_FILENAME: &'static str = ".gazer.toml";
pub const DEFAULT_VALUES: &'static str = r#"
current_version = "0.0.0"

[[files]]
path = "Cargo.toml"
search = "version = \"{current_version}\""
replace = "version = \"{new_version}\""
"#;

#[derive(Clone, Serialize, Deserialize)]
pub struct Config {
    pub current_version: Version,
    files: Vec<FileConfig>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FileConfig {
    path: String,
    search: String,
    replace: String,
}

impl Config {
    pub fn get_current_version(&self) -> &Version {
        &self.current_version
    }

    pub fn get_files(&self) -> &Vec<FileConfig> {
        &self.files
    }
}

impl FileConfig {
    pub fn get_path(&self) -> &String {
        &self.path
    }

    pub fn search_text(&self, current_version: &Version) -> String {
        self.search
            .replace("{current_version}", &current_version.to_string())
    }
    pub fn replace_text(&self, new_version: &Version) -> String {
        self.replace
            .replace("{new_version}", &new_version.to_string())
    }
}

/**
 * Return path of configuration file by inner rule.
 */
pub fn find_config_path() -> Option<PathBuf> {
    let pwd = current_dir().unwrap();
    let _gazer_toml = pwd.join(Path::new(DEFAULT_FILENAME));
    if _gazer_toml.exists() {
        return Some(_gazer_toml);
    }
    None
}

pub fn load_config() -> Option<Config> {
    let config_path = find_config_path();
    if config_path.is_none() {
        return None;
    }
    let config_text = read_to_string(config_path.unwrap()).unwrap();
    Some(toml::from_str(config_text.as_ref()).unwrap())
}
