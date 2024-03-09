use crate::{app, config};
use anyhow::Result;
use semver::Version;

pub fn execute(new_version: &Version) -> Result<()> {
    let init_config = config::load_config().unwrap();

    app::update(&init_config, new_version)
}
