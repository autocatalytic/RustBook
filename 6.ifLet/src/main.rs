fn main() {

    // Sometimes "if let" is better than match
    //
    // Consider
    let some_u8_value = Some(3u8);
    
    match some_u8_value {
        Some(value) => println!("value is {}", value),
        _ => (), // annoying boilerplate code
    }

    // Versus this:
    if let Some(value) = some_u8_value {
        println!("If let value is {}", value);  // More efficient, and no extra work
    }
}

// if let takes a pattern and an expression separated by an equal sign,
// however you lose the exhaustive checking that "match" enforces.
// if let is great if we know we have a single match to make.
