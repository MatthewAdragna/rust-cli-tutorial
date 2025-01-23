use std::{env, error::Error, fs, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();
    match mini_grep::run(&args) {
        Err(err) => panic!("Panic: {err:?}"),
        _ => println!("______SuccessfullOperation_______"),
    };
}

mod mini_grep {

    use super::*;
    pub fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
        let config: Config = Config::new(args)?;

        println!(
            "mini_grep started: [Query: {} | Target: {}]",
            config.query, config.file_path
        );

        let file_path = Path::new(&config.file_path);
        fs::exists(file_path)?;

        let file_text = fs::read_to_string(file_path)?;

        for (index, current_matched) in file_text.match_indices(&config.query) {
            println!("Found {current_matched} @ {index}");
        }

        Ok(())
    }

    struct Config<'a> {
        query: &'a str,
        file_path: &'a str,
    }

    impl<'a> Config<'a> {
        fn new(args: &'a [String]) -> Result<Config<'a>, &'static str> {
            if args.len() < 3 {
                Err("Invalid Argument Amount")
            } else {
                Ok(Config {
                    query: &args[1][..],
                    file_path: &args[2][..],
                })
            }
        }
    }
}
