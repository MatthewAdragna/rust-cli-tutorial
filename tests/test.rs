#[cfg(test)]
mod test {
    //    use super::*;

    #[test]
    fn invalid_args_run() {
        let test_args = vec!["Directory".to_string(), "nothing".to_string()];
        assert!(rct::mini_grep::run2_rawargs(&test_args).is_err());
    }

    // fn print_vec_grep(vecin: Vec<(usize, String)>) {
    //     for (index, line) in vecin {
    //         println!("{index} : {line}");
    //     }
    // }

    #[test]
    fn to_search_empty() {
        let query = "grep";
        let mut to_search = String::from("");
        assert_eq!(rct::mini_grep::grep_string(query, &to_search).len(), 0);
        to_search.push('\n');
        assert_eq!(rct::mini_grep::grep_string(query, &to_search).len(), 0);
    }

    #[test]
    fn query_both_empty() {
        let query = "";
        let to_search = String::from("");
        assert_eq!(rct::mini_grep::grep_string(query, &to_search).len(), 0);
    }

    #[test]
    fn query_empty() {
        let query = "";
        let to_search = String::from("\ngrep\n");
        assert_eq!(rct::mini_grep::grep_string(query, &to_search).len(), 2);
    }

    #[test]
    fn q_on_first_line() {
        let to_search = String::from("grep\n");
        let query = "grep";
        assert_eq!(rct::mini_grep::grep_string(query, &to_search).len(), 1);
    }

    #[test]
    fn q_on_last_line() {
        let to_search = String::from("\ngrep");
        let query = "grep";
        assert_eq!(rct::mini_grep::grep_string(query, &to_search).len(), 1);
    }

    #[test]
    fn normal_behavior() {
        let to_search = String::from("nothingSpecialHere\ngrep\nMovingOn, whats going on over here huh buddy\n grep this shit out broocahcho\n");
        let query = "grep";
        assert_eq!(rct::mini_grep::grep_string(query, &to_search).len(), 2);
    }
}
