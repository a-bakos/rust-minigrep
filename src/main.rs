use std::env;
use std::error::Error;
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

    if let Err(e) = run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

// Box<dyn Error> => trait object
// it means the function will return a type that implements the
// Error trait, but we don't have to specify what particular
// type the return value will be. This gives us flexibility to
// return error values that may be of different types in
// different error cases.
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n{}", contents);

    Ok(())
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
