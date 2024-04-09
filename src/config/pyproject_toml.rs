use std::fs::{read_to_string, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use anyhow::{anyhow, Result};
use toml::de::Error;
use toml::Table;
use toml_edit::{value, DocumentMut};

use super::{Config, ParseAvailable};

pub const FILENAME: &str = "pyproject.toml";

#[derive(Debug)]
pub struct Property {
    pub filepath: PathBuf,
    pub doc: DocumentMut,
}

impl ParseAvailable for Property {
    fn new(root: &Path) -> Result<Self> {
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
        let mut item = &self.doc.to_string().parse::<Table>().unwrap();
        for k in ["tool", "age"] {
            let child = item.get(k);
            if child.is_none() {
                return Err(anyhow!("It does not have valid values."));
            }
            let child = child.unwrap();
            if !child.is_table() {
                return Err(anyhow!("It does not have valid values."));
            }
            item = child.as_table().unwrap();
        }
        let config: Result<Config, Error> = toml::from_str(item.to_string().as_str());
        if config.is_err() {
            return Err(anyhow!(config.unwrap_err()));
        }
        Ok(config.unwrap())
    }

    fn update_version(&mut self, version: &semver::Version) -> Result<()> {
        self.doc["tool"]["age"]["current_version"] = value(version.to_string());
        let mut out = File::create(&self.filepath)?;
        let _ = out.write(self.doc.to_string().as_bytes());
        Ok(())
    }
}
