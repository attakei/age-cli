use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub(crate) struct Arguments {}

pub(crate) fn execute(_args: &Arguments) -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    println!("Running age v{version}");
    Ok(())
}
