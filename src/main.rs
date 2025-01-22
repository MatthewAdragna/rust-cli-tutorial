use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filepath = &args[2];
    let grepped = &args[1];
    println!("Filepath :{filepath:?}");

    println!("ToGrep  :{grepped:?}");

    let file_contents =
        fs::read_to_string(filepath).expect("File could not be read. Program has panicked!");

    println!("File {filepath} has been opened successfully.");
}
