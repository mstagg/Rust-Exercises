use std::env;
use std::process;

use minigrep::Args;

fn main() {
    let args = Args::from(env::args()).unwrap_or_else(|e| {
        eprintln!("Failed to parse arguments: {e}");
        process::exit(1)
    });

    minigrep::run(&args).unwrap_or_else(|e| {
        eprintln!("Application Error: {e}");
        process::exit(1)
    })
}
