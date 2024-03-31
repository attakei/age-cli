mod info;
mod init;
mod major;
mod minor;
mod patch;
mod update;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

use crate::config::resolve_config;

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

pub fn run_command() -> Result<()> {
    let cli = Cli::parse();
    let config = resolve_config();
    match &cli.command {
        Some(Commands::Info(args)) => {
            if config.is_err() {
                return Err(anyhow!(config.unwrap_err()));
            }
            return info::execute(args, &config.unwrap());
        }
        Some(Commands::Init(args)) => {
            return init::execute(args);
        }
        Some(Commands::Update(args)) => {
            if config.is_err() {
                return Err(anyhow!(config.unwrap_err()));
            }
            return update::execute(args, &config.unwrap());
        }
        Some(Commands::Major(args)) => {
            if config.is_err() {
                return Err(anyhow!(config.unwrap_err()));
            }
            return major::execute(args, &config.unwrap());
        }
        Some(Commands::Minor(args)) => {
            if config.is_err() {
                return Err(anyhow!(config.unwrap_err()));
            }
            return minor::execute(args, &config.unwrap());
        }
        Some(Commands::Patch(args)) => {
            if config.is_err() {
                return Err(anyhow!(config.unwrap_err()));
            }
            return patch::execute(args, &config.unwrap());
        }
        None => Ok(()),
    }
}
