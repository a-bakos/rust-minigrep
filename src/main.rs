use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for: {:?}", config.query);
    println!("In file: {:?}", config.filename);

    run(config);
}

fn run(config: Config) {
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
    fn new(args: &Vec<String>) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments!");
        }

        // Easiest though somewhat inefficient way, .clone()
        Ok(Self {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
    }
}
