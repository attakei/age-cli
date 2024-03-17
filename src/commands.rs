mod info;
mod init;
mod major;
mod minor;
mod patch;
mod update;
mod version;

use anyhow::Result;
use clap::{Parser, Subcommand};
use semver::Version;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Display config.
    Info {},
    /// Update version text
    Update { new_version: Version },
    /// Shortcut of major version update
    Major {},
    /// Shortcut of minor version update
    Minor {},
    /// Shortcut of patch version update
    Patch {},
    /// Display version information
    Version {},
    /// Create configuration file.
    Init {},
}

pub fn execute() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Info {}) => info::execute(),
        Some(Commands::Version {}) => version::execute(),
        Some(Commands::Init {}) => init::execute(),
        Some(Commands::Update { new_version }) => update::execute(new_version),
        Some(Commands::Major {}) => major::execute(),
        Some(Commands::Minor {}) => minor::execute(),
        Some(Commands::Patch {}) => patch::execute(),
        None => Ok(()),
    }
}
