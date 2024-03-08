// Entry point.
mod commands;
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
    /// Display version information
    Version {},
    /// Create configuration file.
    Init {},
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Version {}) => {
            commands::version::run();
        }
        Some(Commands::Init {}) => {
            commands::init::run().expect("Command is failed");
        }
        None => {}
    }
}
