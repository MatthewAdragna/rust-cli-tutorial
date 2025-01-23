use rct::mini_grep::run;

use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let index_vec = match run(&args) {
        Err(err) => panic!("Panic: {err:?}"),
        Ok(vec) => vec,
    };

    for index in index_vec {
        println!("Found at {index}")
    }
}
