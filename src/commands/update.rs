use crate::config;
use anyhow::Result;
use semver::Version;
use std::fs::File;
use std::io::prelude::*;
use toml::to_string;

pub fn execute(new_version: &Version) -> Result<()> {
    let config_path = config::find_config_path().unwrap();
    let init_config = config::load_config().unwrap();
    let mut new_config = config::Config::from(init_config);
    new_config.current_version = new_version.clone();

    // Update itself
    let mut out = File::create(config_path)?;
    let _ = out.write(to_string(&new_config).unwrap().as_bytes());

    println!("Updated!!");
    Ok(())
}
