// The Rust Book was refactored between when I did these exercises last time
// so in reviewing my progress I left the original assignments in main-orig.rs
// and started this new main.rs, which covers Mutexes - the common shared
// memory concurrency primitive.
//
//
// Shared-state Concurrency
//

// use std::rc::Rc;             // need Rc<T> to create a reference counted value
                                // oh no, Rc is not thread safe!!
use std::sync::{Arc, Mutex};    // thread safety carries a performance penalty
// use std::sync::Mutex;        // this is now incorrect for thread safety
use std::thread;


// Sharing a Mutex<T> between multiple threads
// 
fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    // let m = Mutex::new(5);

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // call join on each handle to make threads finish
    }

    println!("Result: {}", *counter.lock().unwrap());

//    {
//        let mut num = m.lock().unwrap();
//        *num = 6;
//    }
//
//    println!("m = {:?}", m);
}

// Using this strategy, you can divide a calculation into independent parts,
// split those parts across threads, and thean use a Mutex<T> to have each
// thread update the final result with its part.
//
// Note that very little of Rust is concurrent, therefore concurrency 
// solutions are typically implemented as crates. These evolve more quickly 
// than the standard library, so watch for the current state-of-the-art.
