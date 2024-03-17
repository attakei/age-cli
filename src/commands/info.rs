/* 'info' command displays data from config-file.
 */
use crate::config;
use crate::versioning;

use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub(crate) struct Arguments {}

pub(crate) fn execute(_args: &Arguments) -> Result<()> {
    // Load config file.
    let config_data = config::load_config().unwrap();
    println!("# Version info");
    println!();
    println!("- Current:    {}", config_data.current_version);
    println!(
        "- Next major: {}",
        versioning::up_major(&config_data.current_version)
    );
    println!(
        "- Next minor: {}",
        versioning::up_minor(&config_data.current_version)
    );
    println!(
        "- Next patch: {}",
        versioning::up_patch(&config_data.current_version)
    );
    println!();
    println!("# Replace targets");
    println!();
    for f in config_data.get_files() {
        println!("- {}", f.get_path())
    }
    // Display infomation data.
    Ok(())
}
