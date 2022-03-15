use std::env;
// Refactor to create a library, move fs and Error
//use std::fs;
use std::process;
//use std::error::Error;

use minigrep::Config;
// minigrep is named in Cargo.toml, and Config 
// is a struct within our program library

// Refactoring to use iterators instead of cloning args in lib.rs
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        // 
        // unwrap_or_else lets us define custom non-panic! error handling
        // which allows us to provide more user friendly error messages.
        //
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Because run returns the unit type (), we don't need to use 
    // unwrap_or_else, so we use the "if let" structure to return 
    // information as we only care if we error.
    // 
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {}", e);
        
        process::exit(1);
    }
}

// fn main() {
//     let args: Vec<String> = env::args().collect();  // collect() immediately turns the iterator into 
//     //println!("{:?}", args)                          // a vector containing the values produced by it.

//     let config = Config::new(&args).unwrap_or_else(|err| {
//         eprintln!("Problem parsing arguments: {}", err);
//         process::exit(1);
//     });

//     if let Err(e) = minigrep::run(config) {
//         eprintln!("Application error: {}", e);
        
//         process::exit(1);
//     }
// }
