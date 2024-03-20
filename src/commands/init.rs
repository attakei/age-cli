use crate::commands::require_config;
use crate::config;
use anyhow::{anyhow, Result};
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use clap::Args;

#[derive(Args)]
pub(crate) struct Arguments {}

pub(crate) fn execute(_args: &Arguments) -> Result<()> {
    if require_config().is_ok() {
        return Err(anyhow!("Configuration file is already exists."));
    }
    let target = current_dir()?.join(Path::new(config::DEFAULT_FILENAME));

    println!("Creating file.");
    // Generate config file.
    let mut out = File::create(target)?;
    let conf = config::DEFAULT_VALUES.trim_start().as_bytes();
    let _ = out.write(conf);

    Ok(())
}
