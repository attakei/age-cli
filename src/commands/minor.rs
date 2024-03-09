use crate::{app, config, versioning::up_minor};
use anyhow::Result;

pub fn execute() -> Result<()> {
    let init_config = config::load_config().unwrap();
    let new_version = up_minor(&init_config.current_version);

    app::update(&init_config, &new_version)
}
