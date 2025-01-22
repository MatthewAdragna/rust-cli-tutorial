use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = parse_args(&args);

    println!("Search Query: {}", config.query);

    println!("In file: {}", config.file_path);
}

struct Config {
    query: String,
    file_path: String,
}

fn parse_args(args: &[String]) -> Config {
    if args.len() < 2 {
        panic!("Invalid Arguments")
    }
    Config {
        query: args[1].clone(),
        file_path: args[2].clone(),
    }
}
