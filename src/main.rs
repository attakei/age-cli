// Entry point.
mod commands;
mod config;
mod versioning;
use clap::{Parser, Subcommand};

const VERSION: &'static str = "0.0.0";

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
