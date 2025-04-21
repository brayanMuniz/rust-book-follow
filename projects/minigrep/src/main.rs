use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);

    println!("Query: {}", config.query);
    println!("File path: {}", config.file_path);
    println!("");

    let contents =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("Contents: \n{contents}")
}

#[derive(Debug)]
struct Config {
    query: String,
    file_path: String,
}

impl Config {
    // TODO: usually new is not supposed to fail, change this to build
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!("Not enough arguments!");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Config { query, file_path }
    }
}
