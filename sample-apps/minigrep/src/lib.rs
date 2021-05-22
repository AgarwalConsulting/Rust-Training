use std::fs;
use std::env;
use std::error::Error;

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // }
        // let query = args[1].clone();
        // let filename = args[2].clone();
        // let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        // Ok(Config{query, filename, case_sensitive})

        args.next(); // Discarding the value in 0th index

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("no query string provided")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("no filename provided")
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config{query, filename, case_sensitive})
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // println!("Searching for {}", config.query);
    // println!("In file {}", config.filename);

    let contents = fs::read_to_string(config.filename)?;

    // println!("With text:\n{}", contents);
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line)
    }

    Ok(())
}

// fn parse_config(args: &Vec<String>) -> (String, String) {
//     let query = args[1].clone();
//     let filename = args[2].clone();

//     return (query, filename);
// }

/// Searches for a given query in the given contents and returns the matching lines
///
/// # Examples
///
/// ```
/// let query = "duct";
/// let contents = "\
/// Rust:
/// safe, fast, productive.
/// Pick three.
/// Duct Tape!";
///
/// assert_eq!(vec!["safe, fast, productive."], minigrep::search(query, contents))
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // // let mut results = Vec::new();
    // let mut results = vec!();

    // for line in contents.lines() {
    //     if line.contains(query) {
    //         results.push(line)
    //     }
    // }

    // results
    contents.
        lines().
        filter(|line| line.contains(query)).
        collect()
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = vec!();

    let query = query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line)
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct Tape!";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents))
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust is important!";

        assert_eq!(vec!["Rust:", "Trust is important!"], search_case_insensitive(query, contents))
    }
}
