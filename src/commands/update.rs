use crate::app;
use crate::config::Config;
use anyhow::Result;
use clap::Args;
use semver::Version;

#[derive(Args)]
pub(crate) struct Arguments {
    new_version: Version,
}

pub(crate) fn execute(args: &Arguments, config: &Config) -> Result<()> {
    app::update(&config, &args.new_version)
}
