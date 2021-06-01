use std::env;
use std::fs;

#[warn(unused_variables)]

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let config = Config::new(&args);

    let content = fs::read_to_string(config.filename)
        .expect("Something wrong reading the file.");

    println!("{}", content);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        let query = args[1].clone();
        let filename = args[2].clone();
        Config {query, filename}
    }
}
