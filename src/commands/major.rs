use crate::{app, config, versioning::up_major};
use anyhow::Result;

use clap::Args;

#[derive(Args)]
pub(crate) struct Arguments {}

pub(crate) fn execute(_args: &Arguments) -> Result<()> {
    let init_config = config::load_config().unwrap();
    let new_version = up_major(&init_config.current_version);

    app::update(&init_config, &new_version)
}
