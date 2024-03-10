use anyhow::Result;

pub fn execute() -> Result<()> {
    let version = super::super::VERSION;
    println!("Running age v{version}");
    Ok(())
}
