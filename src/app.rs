use anyhow::Result;
use semver::Version;
use std::fs::{read_to_string, File};
use std::io::prelude::*;
use tera::{Context, Tera};
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
    let ctx = make_context(current_version, new_version);
    for f in new_config.get_files() {
        let search_text = Tera::one_off(&f.search, &ctx, true).unwrap();
        let replace_text = Tera::one_off(&f.replace, &ctx, true).unwrap();
        let mut new_text: Vec<String> = Vec::new();
        for line in read_to_string(&f.path).unwrap().split("\n") {
            if line == search_text {
                new_text.push(replace_text.to_string())
            } else {
                new_text.push(line.to_string())
            }
        }
        let mut out = File::create(&f.path)?;
        let _ = out.write(new_text.join("\n").as_bytes());
    }
    println!("Updated!!");
    Ok(())
}

fn make_context(current_version: &Version, new_version: &Version) -> Context {
    let mut ctx = Context::new();
    ctx.insert("current_version", current_version);
    ctx.insert("new_version", new_version);
    ctx
}
