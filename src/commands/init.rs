use std::{env::current_dir, path::Path, result::Result};

const DEFAULT_CONFIG_FILE: &'static str = ".gazer.toml";

pub fn run() -> Result<(), &'static str> {
    // TODO: Right way?
    let target = current_dir()
        .expect("Failed current directory")
        .join(Path::new(DEFAULT_CONFIG_FILE));

    {
        // TODO: Right way?
        let target = target.as_os_str().to_str().expect("Failure");
        println!("{target}");
    }

    if target.exists() {
        println!("Config file is already exists.");
    } else {
        // Generate config file.
    }

    Ok(())
}
