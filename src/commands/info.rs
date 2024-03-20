/* 'info' command displays data from config-file.
 */
use crate::commands::require_config;
use crate::versioning;

use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub(crate) struct Arguments {}

pub(crate) fn execute(_args: &Arguments) -> Result<()> {
    let config = require_config();
    if config.is_err() {
        return Err(config.unwrap_err());
    }
    let config = config.unwrap();
    println!("# Version info");
    println!();
    println!("- Current:    {}", config.current_version);
    println!(
        "- Next major: {}",
        versioning::up_major(&config.current_version)
    );
    println!(
        "- Next minor: {}",
        versioning::up_minor(&config.current_version)
    );
    println!(
        "- Next patch: {}",
        versioning::up_patch(&config.current_version)
    );
    println!();
    println!("# Replace targets");
    println!();
    for f in config.get_files() {
        println!("- {}", f.path.to_str().unwrap())
    }
    // Display infomation data.
    Ok(())
}
