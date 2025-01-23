pub mod mini_grep {

    use std::{error::Error, fs, path::Path};
    pub fn run(args: &[String]) -> Result<Vec<usize>, Box<dyn Error>> {
        let config: Config = Config::new(args)?;

        println!(
            "mini_grep started: [Query: {} | Target: {}]",
            config.query, config.file_path
        );

        let file_path = Path::new(&config.file_path);
        fs::exists(file_path)?;

        let file_text = fs::read_to_string(file_path)?;
        let mut outputs: Vec<usize> = Vec::new();
        for (index, _) in file_text.match_indices(&config.query) {
            outputs.push(index);
        }

        Ok(outputs)
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
