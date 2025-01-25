use rct::mini_grep::run2_rawargs;

use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let index_vec = match run2_rawargs(&args) {
        Err(err) => panic!("Panic: {err:?}"),
        Ok(vec) => vec,
    };

    for (num, line) in index_vec {
        println!("{num} : {line}");
    }
}
