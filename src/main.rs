use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config: Config = Config::new(&args);

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
impl Config {
    // &Vec<String> = &[String]
    fn new(args: &Vec<String>) -> Self {
        if args.len() < 3 {
            panic!("Not enough arguments!");
        }

        // Easiest though somewhat inefficient way, .clone()
        Self {
            query: args[1].clone(),
            filename: args[2].clone(),
        }
    }
}
