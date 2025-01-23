use rct::mini_grep::run;

use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    match run(&args) {
        Err(err) => panic!("Panic: {err:?}"),
        _ => println!("Successfully completed mini_grep operation"),
    };
}
