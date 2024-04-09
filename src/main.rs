// Entry point.
use crate::commands::run_command;

mod commands;
mod config;
mod versioning;
mod workspace;
mod writer;

fn main() {
    env_logger::init();
    let result = run_command();
    let return_code = match result {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("{err}");
            1
        }
    };
    std::process::exit(return_code);
}
