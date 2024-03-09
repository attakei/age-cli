use crate::{app, config, versioning::up_patch};
use anyhow::Result;

pub fn execute() -> Result<()> {
    let init_config = config::load_config().unwrap();
    let new_version = up_patch(&init_config.current_version);

    app::update(&init_config, &new_version)
}
