use minigrep_project::Config;
use std::env;
use std::process;

fn main() {
    // env::args() returns an iterator
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // println!("Searching for: {:?}", config.query);
    // println!("In file: {:?}", config.filename);

    if let Err(e) = minigrep_project::run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
