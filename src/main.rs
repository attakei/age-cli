// Entry point.
mod app;
mod commands;
mod config;
mod versioning;
use crate::commands::run_command;

fn main() {
    let result = run_command();
    match result {
        Ok(()) => 0,
        Err(err) => {
            println!("{err}");
            1
        }
    };
}
