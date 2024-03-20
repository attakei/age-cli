use crate::commands::require_config;
use crate::{app, versioning::up_patch};
use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub(crate) struct Arguments {}

pub(crate) fn execute(_args: &Arguments) -> Result<()> {
    let config = require_config();
    if config.is_err() {
        return Err(config.unwrap_err());
    }
    let init_config = config.unwrap();
    let new_version = up_patch(&init_config.current_version);

    app::update(&init_config, &new_version)
}
