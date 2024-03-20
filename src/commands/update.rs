use crate::app;
use crate::commands::require_config;
use anyhow::Result;
use clap::Args;
use semver::Version;

#[derive(Args)]
pub(crate) struct Arguments {
    new_version: Version,
}

pub(crate) fn execute(args: &Arguments) -> Result<()> {
    let config = require_config();
    if config.is_err() {
        return Err(config.unwrap_err());
    }
    let init_config = config.unwrap();

    app::update(&init_config, &args.new_version)
}
