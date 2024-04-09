use std::fs::{read_to_string, File};
use std::io::prelude::*;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use toml_edit::de::from_document;
use toml_edit::{value, DocumentMut};

use super::{Config, ParseAvailable};

pub const FILENAME: &'static str = ".age.toml";

#[derive(Debug)]
pub struct Property {
    pub filepath: PathBuf,
    pub doc: DocumentMut,
}

impl ParseAvailable for Property {
    fn new(root: &PathBuf) -> Result<Self> {
        let filepath = root.join(FILENAME);
        if !filepath.exists() {
            return Err(anyhow!("Configuration file is not found."));
        }
        let source = read_to_string(&filepath);
        if source.is_err() {
            return Err(anyhow!("Configuration file cannot access."));
        }
        let doc = source.unwrap().parse::<DocumentMut>();
        if doc.is_err() {
            return Err(anyhow!("Configuration is not valid TOML."));
        }

        Ok(Property {
            filepath,
            doc: doc.unwrap(),
        })
    }

    fn get_config(&self) -> Result<Config> {
        let config = from_document::<Config>(self.doc.clone());
        if config.is_err() {
            return Err(anyhow!(config.unwrap_err()));
        }
        Ok(config.unwrap())
    }

    fn update_version(&mut self, version: &semver::Version) -> Result<()> {
        self.doc["current_version"] = value(version.to_string());
        let mut out = File::create(&self.filepath)?;
        let _ = out.write(self.doc.to_string().as_bytes());
        Ok(())
    }
}
