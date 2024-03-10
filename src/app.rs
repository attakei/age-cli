use anyhow::Result;
use semver::Version;
use std::fs::{read_to_string, File};
use std::io::prelude::*;
use toml::to_string;

use crate::config::{find_config_path, Config};

pub fn update(base_config: &Config, new_version: &Version) -> Result<()> {
    let mut new_config = base_config.clone();
    new_config.current_version = new_version.clone();

    // Update itself
    let mut out = File::create(find_config_path().unwrap())?;
    let _ = out.write(to_string(&new_config).unwrap().as_bytes());

    // Update for '[[files]]' targets
    let current_version = &base_config.current_version;
    for f in new_config.get_files() {
        let search_text = f.search_text(current_version);
        let mut new_text: Vec<String> = Vec::new();
        for line in read_to_string(f.get_path()).unwrap().split("\n") {
            if line == search_text {
                new_text.push(f.replace_text(&new_version))
            } else {
                new_text.push(line.to_string())
            }
        }
        let mut out = File::create(f.get_path())?;
        let _ = out.write(new_text.join("\n").as_bytes());
    }
    println!("Updated!!");
    Ok(())
}
