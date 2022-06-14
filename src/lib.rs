use std::error::Error;
use std::fs;

// Box<dyn Error> => trait object
// it means the function will return a type that implements the
// Error trait, but we don't have to specify what particular
// type the return value will be. This gives us flexibility to
// return error values that may be of different types in
// different error cases.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let _contents = fs::read_to_string(config.filename)?;
    // println!("With text:\n{}", contents);
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
}
impl Config {
    // &Vec<String> = &[String]
    pub fn new(args: &Vec<String>) -> Result<Self, &'static str> {
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
