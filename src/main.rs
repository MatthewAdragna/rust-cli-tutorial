use std::{env, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::new(&args).unwrap_or_else(|x| {
        panic!("Panic: |Parsing arguments| {x}");
    });

    println!("Search Query: {}", config.query);
    println!("In file: {}", config.file_path);
    let file_path = Path::new(&config.file_path);
    fs::exists(file_path).expect(" {file_path:?} does not exist!");

    let file_text =
        fs::read_to_string(file_path).expect("Panic: file could not be converted to text.");
    for (index, current_matched) in file_text.match_indices(&config.query) {
        println!("Found {current_matched} @ {index}");
    }
}

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            Err("Invalid Argument Amount")
        } else {
            Ok(parse_args(args))
        }
    }
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
