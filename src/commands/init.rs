use crate::commands::require_config;
use crate::config;
use anyhow::{anyhow, Result};
use std::env::current_dir;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use clap::Args;
use tera::{Context, Tera};

const TEMPLATE_BASE: &'static str = r#"
current_version = "{{ current_version }}"

{{ files }}
"#;

const TEMPLATE_FOR_RUST: &'static str = r#"
[[files]]
path = "Cargo.toml"
search = "version = \"{{current_version}}\""
replace = "version = \"{{new_version}}\""
"#;

const TEMPLATE_FOR_PYTHON: &'static str = r#"
[[files]]
path = "pyproject.toml"
search = "version = \"{{current_version}}\""
replace = "version = \"{{new_version}}\""
"#;

#[derive(Args)]
pub(crate) struct Arguments {
    #[arg(long, value_parser=["rust", "python"], default_value="rust")]
    preset: String,
}

pub(crate) fn execute(args: &Arguments) -> Result<()> {
    if require_config().is_ok() {
        return Err(anyhow!("Configuration file is already exists."));
    }
    let target = current_dir()?.join(Path::new(config::DEFAULT_FILENAME));

    let mut ctx = Context::new();
    ctx.insert("current_version", "0.0.0");
    match args.preset.as_str() {
        "rust" => ctx.insert("files", &TEMPLATE_FOR_RUST.to_string().trim_start()),
        "python" => ctx.insert("files", &TEMPLATE_FOR_PYTHON.to_string().trim_start()),
        p => {
            println!("'{}' is not support", p);
            ctx.insert("files", &"".to_string());
        }
    }

    println!("Creating file.");
    // Generate config file.
    let mut out = File::create(target)?;
    let _ = out.write(
        Tera::one_off(TEMPLATE_BASE.trim_start(), &ctx, false)
            .unwrap()
            .as_bytes(),
    );
    Ok(())
}
