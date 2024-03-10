// Entry point.
mod app;
mod commands;
mod config;
mod versioning;
use clap::{Parser, Subcommand};
use semver::Version;

const VERSION: &'static str = "0.1.0";

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

fn main() {
    let cli = Cli::parse();
    let result = match &cli.command {
        Some(Commands::Info {}) => commands::info::execute(),
        Some(Commands::Version {}) => commands::version::execute(),
        Some(Commands::Init {}) => commands::init::execute(),
        Some(Commands::Update { new_version }) => commands::update::execute(new_version),
        Some(Commands::Major {}) => commands::major::execute(),
        Some(Commands::Minor {}) => commands::minor::execute(),
        Some(Commands::Patch {}) => commands::patch::execute(),
        None => Ok(()),
    };
    match result {
        Ok(()) => 0,
        Err(err) => {
            println!("{err}");
            1
        }
    };
}
