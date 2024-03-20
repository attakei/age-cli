mod info;
mod init;
mod major;
mod minor;
mod patch;
mod update;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

use crate::config;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Display config.
    Info(info::Arguments),
    /// Update version text
    Update(update::Arguments),
    /// Shortcut of major version update
    Major(major::Arguments),
    /// Shortcut of minor version update
    Minor(minor::Arguments),
    /// Shortcut of patch version update
    Patch(patch::Arguments),
    /// Create configuration file.
    Init(init::Arguments),
}

/**
 * Resolve valid configuration file and parse struct.
 */
fn require_config() -> Result<config::Config> {
    if config::find_config_path().is_none() {
        return Err(anyhow!("Configuratio file is not exists."));
    }
    return Ok(config::load_config().unwrap());
}

pub fn run_command() -> Result<()> {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Info(args)) => info::execute(args),
        Some(Commands::Init(args)) => init::execute(args),
        Some(Commands::Update(args)) => update::execute(args),
        Some(Commands::Major(args)) => major::execute(args),
        Some(Commands::Minor(args)) => minor::execute(args),
        Some(Commands::Patch(args)) => patch::execute(args),
        None => Ok(()),
    }
}
