// Entry point.
mod app;
mod commands;
mod config;
mod versioning;
use crate::commands::execute;

fn main() {
    let result = execute();
    match result {
        Ok(()) => 0,
        Err(err) => {
            println!("{err}");
            1
        }
    };
}
