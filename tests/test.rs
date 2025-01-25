use rand::Rng;

#[cfg(test)]
mod test {
    //    use super::*;

    #[test]
    fn invalid_args_run() {
        let test_args = vec!["Directory".to_string(), "nothing".to_string()];
        assert!(rct::mini_grep::run2_rawargs(&test_args).is_err());
    }

    fn printThatfknVec(vecin: Vec<(usize, String)>) {
        for (index, line) in vecin {
            println!("{index} : {line}");
        }
    }

    #[test]
    fn to_search_empty() {}

    #[test]
    fn query_empty() {}

    #[test]
    fn q_on_first_line() {}

    #[test]
    fn q_on_last_line() {}

    #[test]
    fn normal_behavior() {}
}
