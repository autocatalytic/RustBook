use std::env;
// Refactor to create a library, move fs and Error
//use std::fs;
use std::process;
//use std::error::Error;

use minigrep::Config;

// Refactoring to use iterators instead of cloning args in lib.rs
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

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
