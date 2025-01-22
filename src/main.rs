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
    if args.len() < 3 {
        panic!("Invalid Arguments Amount Found, Found {} arguments\n Minimum amount of arguments needed is 2  in form :m{{Query,File_Path}}",args.len()-1);
    }
    Config {
        query: args[1].clone(),
        file_path: args[2].clone(),
    }
}
