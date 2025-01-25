pub mod mini_grep {

    use std::{error::Error, fs, path::Path};
    fn run(config: &Config) -> Result<Vec<usize>, Box<dyn Error>> {
        println!(
            "mini_grep started: [Query: {} | Target: {}]",
            config.query, config.file_path
        );

        let file_path = Path::new(config.file_path);
        fs::exists(file_path)?;

        let file_text = fs::read_to_string(file_path)?;
        let mut outputs: Vec<usize> = Vec::new();
        for (index, _) in file_text.match_indices(config.query) {
            outputs.push(index);
        }

        Ok(outputs)
    }

    pub fn run1_rawargs(args: &[String]) -> Result<Vec<usize>, Box<dyn Error>> {
        let config: Config = Config::new(args)?;
        run(&config)
    }

    pub fn run2_rawargs(args: &[String]) -> Result<Vec<(usize, String)>, Box<dyn Error>> {
        let config: Config = Config::new(args)?;
        run2(&config)
    }

    fn run2(config: &Config) -> Result<Vec<(usize, String)>, Box<dyn Error>> {
        println!(
            "mini_grep started: [Query: {} | Target: {}]",
            config.query, config.file_path
        );

        let file_path = Path::new(config.file_path);
        fs::exists(file_path)?;
        let file_text = fs::read_to_string(file_path)?;
        Ok(grep_string(config.query, &file_text))
    }

    pub fn grep_string(query: &str, file_text: &str) -> Vec<(usize, String)> {
        let filter = file_text
            .lines()
            .enumerate()
            .filter(|(_, linein)| linein.contains(query));

        filter
            .map(|(num, slice_in)| (num, String::from(slice_in)))
            .collect()
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
