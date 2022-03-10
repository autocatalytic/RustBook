// Enums allow you to define a type by enumerating its possible variants
//
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) -> (String) {
        //method body would be defined here
        let x = "World";
        return x.to_string()
    }
}

// fn main() {
//     let m = Message::Write(String::from("hello"));
//     // I can't figure out how to get m setup for default println!
//     // so have to use debug.  I'll figure it out eventually and don't
//     // want to get bogged down here. Enum's can have methods. Got it.
//     println!("Message: {:?} {}", m, m.call());
  
// }

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!"); 
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
// That's cool syntax
//
// Option<T>
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(five);

    println!("{:?}", six);
}
