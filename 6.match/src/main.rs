// Let's use enums with match!
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,       // The => operator separates the pattern and the code
        Coin::Nickel => 5,      // in this case the expression is the number 1 :-)
        Coin::Dime => 10,       
        Coin::Quarter => 25,    // Each of these match lines is an "arm".
    }
}

// You can also have code in the match arm, using curly braces, like this:
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {       // The => operator separates the pattern and the code
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,      // in this case the expression is the number 1 :-)
        Coin::Dime => 10,       
        Coin::Quarter => 25,    // Each of these match lines is an "arm".
    }
}

// We can also place data inside the enums, that match patterns of related enums
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // -- snip --
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}



fn main() {
    println!("Hello, world!");
}
