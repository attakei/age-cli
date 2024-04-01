use crate::workspace::{make_context, Workspace};
use anyhow::Result;
use clap::Args;
use semver::Version;

#[derive(Args)]
pub(crate) struct Arguments {
    new_version: Version,
}

pub(crate) fn execute(args: &Arguments, workspace: &Workspace) -> Result<()> {
    let current_version = &workspace.config.current_version;
    let context = make_context(&current_version, &args.new_version);
    workspace.update_files(&context)
}
