use rct::mini_grep::run2_rawargs;
use std::env;
fn main() {
    match env::var("funnyEnvironmentalVariable") {
        Ok(a) => println!("Funny little Environmental variable is set to {a}"),
        Err(a) => println!("Funny little Environmental Variable was not set :C  {a}"),
    }

    let args: Vec<String> = env::args().collect();
    let index_vec = match run2_rawargs(&args) {
        Err(err) => panic!("Panic: {err:?}"),
        Ok(vec) => vec,
    };

    for (num, line) in index_vec {
        println!("{num} : {line}");
    }
}
