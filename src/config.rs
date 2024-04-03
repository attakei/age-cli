// Configuration manager.

use anyhow::{anyhow, Result};
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

mod age_toml;
mod cargo_toml;

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

pub trait ParseAvaliable {
    fn new(root: &PathBuf) -> Result<Self>
    where
        Self: Sized;
    fn get_config(&self) -> Result<Config>;
    fn update_version(&mut self, version: &Version) -> Result<()>;
}

#[derive(Debug)]
pub enum ConfigDocument {
    AgeToml(age_toml::Property),
    CargoToml(cargo_toml::Property),
}

impl ConfigDocument {
    pub fn filename(&self) -> String {
        match self {
            Self::AgeToml(_) => age_toml::FILENAME.to_string(),
            Self::CargoToml(_) => cargo_toml::FILENAME.to_string(),
        }
    }
    pub fn update_version(&mut self, version: &Version) -> Result<()> {
        match self {
            Self::AgeToml(props) => props.update_version(version),
            Self::CargoToml(props) => props.update_version(version),
        }
    }
}

pub fn resolve_config(root: &PathBuf) -> Result<(ConfigDocument, Config)> {
    let _age_toml = '_age_toml: {
        let doc = age_toml::Property::new(root);
        if doc.is_err() {
            break '_age_toml Err(doc.unwrap_err());
        }
        let doc = doc.unwrap();
        let config = doc.get_config();
        if config.is_err() {
            break '_age_toml Err(config.unwrap_err());
        }
        Ok((ConfigDocument::AgeToml(doc), config.unwrap()))
    };
    if _age_toml.is_ok() {
        return _age_toml;
    }
    let _cargo_toml = '_cargo_toml: {
        let doc = cargo_toml::Property::new(root);
        if doc.is_err() {
            break '_cargo_toml Err(doc.unwrap_err());
        }
        let doc = doc.unwrap();
        let config = doc.get_config();
        if config.is_err() {
            break '_cargo_toml Err(config.unwrap_err());
        }
        Ok((ConfigDocument::CargoToml(doc), config.unwrap()))
    };
    if _cargo_toml.is_ok() {
        return _cargo_toml;
    }
    Err(anyhow!("Valid configuration file is not exists."))
}
