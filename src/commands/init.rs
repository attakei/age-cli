use crate::config;
use anyhow::{anyhow, Result};
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn execute() -> Result<()> {
    let target = current_dir()?.join(Path::new(config::DEFAULT_FILENAME));
    {
        // TODO: Right way?
        let target = target.as_os_str().to_str().expect("Failure");
        println!("Config file is {target}");
    }

    if target.exists() {
        // println!("Already exists.");
        return Err(anyhow!("Already exists."));
    }
    println!("Creating file.");
    // Generate config file.
    let mut out = File::create(target)?;
    let conf = config::DEFAULT_VALUES.as_bytes();
    let _ = out.write(conf);

    Ok(())
}
