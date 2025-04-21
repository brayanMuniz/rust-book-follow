use std::{env, process};

use minigrep::{Config, run};

fn main() {
    let args: Vec<String> = env::args().collect();

    // return Ok if ok, else use the closure
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arrguments {err}");
        process::exit(1);
    });

    println!("Query: {}", config.query);
    println!("File path: {}", config.file_path);
    println!("");

    if let Err(e) = run(config) {
        println!("Application failed to run: {e}");
        process::exit(1);
    }
}
