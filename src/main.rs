// Entry point.
mod app;
mod commands;
mod config;
mod versioning;
mod writer;
use crate::commands::run_command;

fn main() {
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
