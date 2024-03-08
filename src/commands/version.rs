use anyhow::Result;

pub fn execute() -> Result<()> {
    let version = super::super::VERSION;
    println!("Running gazer v{version}");
    Ok(())
}
