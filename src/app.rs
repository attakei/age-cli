use anyhow::Result;
use chrono::Local;
use semver::Version;
use std::path::PathBuf;
use tera::Context;

use crate::config::{Config, DEFAULT_FILENAME};
use crate::writer::Writer;

pub fn update(base_config: &Config, new_version: &Version) -> Result<()> {
    let mut new_config = base_config.clone();
    new_config.current_version = new_version.clone();

    let ctx = make_context(&base_config.current_version, new_version);
    let mut writer = Writer::new(&ctx);
    writer.add_target(
        &PathBuf::from(DEFAULT_FILENAME),
        &("current_version = \"{{current_version}}\"".to_string()),
        &("current_version = \"{{new_version}}\"".to_string()),
    );
    for f in base_config.get_files() {
        writer.add_target(&f.path, &f.search, &f.replace);
    }

    match writer.update_all() {
        Ok(_) => {
            println!("Updated!!");
            Ok(())
        }
        Err(err) => Err(err),
    }
}

fn make_context(current_version: &Version, new_version: &Version) -> Context {
    let mut ctx = Context::new();
    ctx.insert("current_version", current_version);
    ctx.insert("new_version", new_version);
    ctx.insert("now", &Local::now().to_rfc3339());
    return ctx;
}
