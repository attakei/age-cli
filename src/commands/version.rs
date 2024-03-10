use anyhow::Result;

pub fn execute() -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    println!("Running age v{version}");
    Ok(())
}
