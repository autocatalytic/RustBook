use crate::List::{Cons, Nil};

// The "cons list" comes from Lisp and its dialects, and is short for "construct
// function". It constructs a new pair from its two arguments, which are usually
// a single value and another pair. These pairs containing pairs form a list.
//
// Note that in rust, Vec<T> is better than lists of items in most cases.
//  
enum List {
    Cons(i32, Box<List>),   // If we don't Box List, we get an infinite size error
    Nil,                    // Box gives indirection (pointer) and heap allocation
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Building our first smart pointer, similar to the Box<T> type in std lib.
// 
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;

impl<T> Deref for MyBox<T> {    // Elegant way to grant a std trait to our
    type Target = T;            // custom Box type.

    fn deref(&self) -> &Self::Target {  // Target is an associated type
        &self.0                         // similar to a generic parameter
    }
}

// Not so interesting because it is a single value. More useful would be a 
// recursive function or type, because it would be undefined in size.
// 
// fn main() {
//     let b  = Box::new(5);
//     println!("b = {}", b);
// }

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    let x = 5; 
//    let y = &x; 
//    let y = Box::new(x);    // set y equal to a boxed copy of x
    let y = MyBox::new(x);  // now let's use the custom tuple MyBox, which 
                            // then requires us to implement the "deref" 
                            // trait for our struct.

    assert_eq!(5, x);
    assert_eq!(5, *y);  // y and 5 are different T's. y is a pointer to a number

    fn hello(name: &str) {
        println!("Hello, {}!", name);
    }

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    todo!("Reference counted smart pointers, interior mutability, & ref. cycles");
}
// This nil is different than prior. It's a non-recursive variant that signals
// the end of the list.

