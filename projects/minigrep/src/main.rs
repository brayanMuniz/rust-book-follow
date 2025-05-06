use std::{env, process};

use minigrep::{Config, run};

fn main() {
    // env:args() is an iterator
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arrguments {err}");
        process::exit(1);
    });

    println!("Query: {}", config.query);
    println!("File path: {}", config.file_path);
    println!("");

    if let Err(e) = run(config) {
        eprintln!("Application failed to run: {e}");
        process::exit(1);
    }
}
