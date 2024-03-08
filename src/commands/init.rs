use std::{env::current_dir, fs::File, path::Path, result::Result};

use std::io::prelude::*;

const DEFAULT_CONFIG_FILE: &'static str = ".gazer.toml";
const DEFAULT_CONFIG_VALUE: &'static str = r#"
current_version = "0.0.0"

[[files]]
path = "Cargo.toml"
search = "version = \"{current_version}\""
replace = "version = \"{new_version}\""
"#;

pub fn run() -> Result<(), &'static str> {
    // TODO: Right way?
    let target = current_dir()
        .expect("Failed current directory")
        .join(Path::new(DEFAULT_CONFIG_FILE));

    {
        // TODO: Right way?
        let target = target.as_os_str().to_str().expect("Failure");
        println!("Config file is {target}");
    }

    if target.exists() {
        println!("Already exists.");
    } else {
        println!("Creating file.");
        // Generate config file.
        let mut out = File::create(target).expect("File-creating is failed");
        let conf = DEFAULT_CONFIG_VALUE.as_bytes();
        let _ = out.write(conf);
    }

    Ok(())
}
