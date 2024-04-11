// Configuration manager.

use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use log::info;
use semver::Version;
use serde::{Deserialize, Serialize};

pub mod age_toml;
mod cargo_toml;
mod pyproject_toml;

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
    pub regex: Option<bool>,
}

impl FileConfig {
    pub fn regex(&self) -> bool {
        self.regex.is_some() && self.regex.unwrap()
    }
}

pub trait ParseAvailable {
    fn new(root: &Path) -> Result<Self>
    where
        Self: Sized;
    fn get_config(&self) -> Result<Config>;
    fn update_version(&mut self, version: &Version) -> Result<()>;
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum ConfigDocument {
    AgeToml(age_toml::Property),
    CargoToml(cargo_toml::Property),
    PyprojectToml(pyproject_toml::Property),
}

impl ConfigDocument {
    pub fn filename(&self) -> String {
        match self {
            Self::AgeToml(_) => age_toml::FILENAME.to_string(),
            Self::CargoToml(_) => cargo_toml::FILENAME.to_string(),
            Self::PyprojectToml(_) => pyproject_toml::FILENAME.to_string(),
        }
    }
    pub fn update_version(&mut self, version: &Version) -> Result<()> {
        match self {
            Self::AgeToml(props) => props.update_version(version),
            Self::CargoToml(props) => props.update_version(version),
            Self::PyprojectToml(props) => props.update_version(version),
        }
    }
}

pub fn resolve_config(root: &Path) -> Result<(ConfigDocument, Config)> {
    let _age_toml = '_age_toml: {
        let doc = age_toml::Property::new(root);
        if let Err(err) = doc {
            break '_age_toml Err(err);
        }
        let doc = doc.unwrap();
        let config = doc.get_config();
        if let Err(err) = config {
            break '_age_toml Err(err);
        }
        Ok((ConfigDocument::AgeToml(doc), config.unwrap()))
    };
    if let Ok(result) = _age_toml {
        info!("Found valid .age.toml");
        return Ok(result);
    }
    let _cargo_toml = '_cargo_toml: {
        let doc = cargo_toml::Property::new(root);
        if let Err(err) = doc {
            break '_cargo_toml Err(err);
        }
        let doc = doc.unwrap();
        let config = doc.get_config();
        if let Err(err) = config {
            break '_cargo_toml Err(err);
        }
        Ok((ConfigDocument::CargoToml(doc), config.unwrap()))
    };
    if _cargo_toml.is_ok() {
        info!("Found valid Cargo.toml");
        return _cargo_toml;
    }
    let _pyproject_toml = '_pyproject_toml: {
        let doc = pyproject_toml::Property::new(root);
        if let Err(err) = doc {
            break '_pyproject_toml Err(err);
        }
        let doc = doc.unwrap();
        let config = doc.get_config();
        if let Err(err) = config {
            break '_pyproject_toml Err(err);
        }
        Ok((ConfigDocument::PyprojectToml(doc), config.unwrap()))
    };
    if _pyproject_toml.is_ok() {
        info!("Found valid pyproject.toml");
        return _pyproject_toml;
    }
    info!(
        "There is not valid configuration in {}",
        root.display().to_string()
    );
    Err(anyhow!("Valid configuration file is not exists."))
}
