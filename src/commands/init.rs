use anyhow::{anyhow, Result};
use std::{env::current_dir, fs::File, path::Path};

use std::io::prelude::*;

const DEFAULT_CONFIG_FILE: &'static str = ".gazer.toml";
const DEFAULT_CONFIG_VALUE: &'static str = r#"
current_version = "0.0.0"

[[files]]
path = "Cargo.toml"
search = "version = \"{current_version}\""
replace = "version = \"{new_version}\""
"#;

pub fn execute() -> Result<()> {
    let target = current_dir()?.join(Path::new(DEFAULT_CONFIG_FILE));
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
    let conf = DEFAULT_CONFIG_VALUE.as_bytes();
    let _ = out.write(conf);

    Ok(())
}
