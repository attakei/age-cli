use crate::{app, config};
use anyhow::Result;
use clap::Args;
use semver::Version;

#[derive(Args)]
pub(crate) struct Arguments {
    new_version: Version,
}

pub(crate) fn execute(args: &Arguments) -> Result<()> {
    let init_config = config::load_config().unwrap();

    app::update(&init_config, &args.new_version)
}
