use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = parse_config(&args);

    println!("Searching for: {:?}", config.query);
    println!("In file: {:?}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file!");

    println!("With text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

// &Vec<String> = &[String]
fn parse_config(args: &Vec<String>) -> Config {
    Config {
        query: args[1].clone(),
        filename: args[2].clone(),
    }
}
