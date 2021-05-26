use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("searching for instances of {}", query);
    println!("in file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("problem reading file");
    
    println!("with text:\n{}", contents);
}