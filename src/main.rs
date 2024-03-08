// Entry point.
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
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Version {}) => {
            println!("Running gazer v{VERSION}");
        }
        None => {}
    }
}
