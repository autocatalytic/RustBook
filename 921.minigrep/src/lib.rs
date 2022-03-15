use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        // if args.len() < 3 {
        //     return Err("not enough arguments");
        // } else {
        //     if args.len() > 3 {
        //         return Err("too many arguments");
        //     }
        // }

        // To make the program simple to write on the first attempt, use clone()
        // It's easier because new() doesn't own args, and taking
        // a slice of String elements requires ownership.
        //
        // let query = args[1].clone();
        // let filename = args[2].clone();

        // Refactor the above to use iterators instead of cloning
        // First change main.rs to pass through the original arguments not a vec
        // Then change signature to accept args from std::env::Args, adding the 
        // mut keyword so we can mutate args and iterate over it

        args.next();    // First return val of env::args is program name, discard

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { 
            query, 
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
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


// Refactor search() to use iterators, look how efficient and clear!
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}
// pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
//     let mut results = Vec::new();       // mutable vector to store results

//     for line in contents.lines() {    // really handy method: lines -> Iter
//         if line.contains(query) {           // another excellent method: contains -> String
//             results.push(line);
//         }
//     }

//     results
// }

pub fn search_case_insensitive<'a>(
    query: &str, 
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {      
        // contains() takes string slice, so need borrow query here:
        if line.to_lowercase().contains(&query) {   
            results.push(line);
        }
    }

    results
}
