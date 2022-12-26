use std::env;
use std::process;

use minigrep::Args;

fn main() {
    let args: Vec<String> = env::args().collect();

    let args = Args::from(&args).unwrap_or_else(|e| {
        eprintln!("Failed to parse arguments: {e}");
        process::exit(1)
    });

    minigrep::run(&args).unwrap_or_else(|e| {
        eprintln!("Application Error: {e}");
        process::exit(1)
    })
}
