#[cfg(test)]
mod test {
    //    use super::*;

    #[test]
    fn invalid_args_run() {
        let test_args = vec!["Directory".to_string(), "nothing".to_string()];
        assert!(rct::mini_grep::run(&test_args).is_err());
    }
}
