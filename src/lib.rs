use std::env;
use std::error::Error;
use std::fs;

// Box<dyn Error> => trait object
// it means the function will return a type that implements the
// Error trait, but we don't have to specify what particular
// type the return value will be. This gives us flexibility to
// return error values that may be of different types in
// different error cases.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    // println!("With text:\n{}", contents);

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    // &Vec<String> = &[String]
    pub fn new(mut args: std::env::Args) -> Result<Self, &'static str> {
        // The first value in the return of env::args is the name of the
        // program, so we want to skip that
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        // Easiest though somewhat inefficient way, .clone()
        Ok(Self {
            query,
            filename,
            case_sensitive,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}

/**
 * We need an explicit lifetime of 'a defined in the signature and used with
 * the contents argument and the return value. The lifetime parameters
 * specify which argument lifetime is connected to the lifetime of the return
 * value.
 * We indicate that the returned vector should contain string slices that
 * reference slices of the argument contents - rather than query.
 * In other words, we tell Rust, that the data returned by the search function
 * will live as long as the data passed into the function in the contents
 * argument.
 * The data referenced by a slice needs to be valid for the reference to be
 * valid. If the compiler assumes we're making string slices of query rather
 * than contents, it will do its safety checking incorrectly.
 * Rust can't possibly know which of the two arguments we need, so we need to
 * tell it. Because contents is the argument that contains all of our text and
 * we want to return the parts of that text that match, contents is the
 * argument that should be connected to the return value using the lifetime
 * syntax.
 */
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()

    // The old way for the above code, aka using no iterators
    /*
    let mut matches: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            matches.push(line);
        }
    }
    matches
    */
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()

    // The old way for the above code, aka using no iterators
    /*
    let mut matches: Vec<&str> = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            matches.push(line);
        }
    }
    matches
    */
}
