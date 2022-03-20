use std::sync::mpsc;
use std::thread;

// Rust implements 1:1 threading, because it chooses to have as small a 
// runtime as possible. More green threads means larger runtime. In the rust
// context, with multi-threading, runtime means the code that is included by
// the language in every binary.
// 
// However, because rust is such a low-level language, tthere are crates that 
// imlement M:N threading, at the expense of overhead for this feature
// 

// fn main() {
//     let handle = thread::spawn(|| { // type returned is JoinHandle. It's an 
//                                     // owned value that will wait for its 
//                                     // thread to finish.
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
// 
//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     } 
// 
//     handle.join().unwrap();  // unwrap makes it panic in case of an error
// }

// Ensuring safe concurrency with message passing, where threads or actors
// communicate by sending eachother messages containing data. From Go docs:
// "Do not communicate by sharing memory; instead, share memory by communicating."
//
// Introducing channels!
// 
// fn main() {
//     let (tx, rx) = mpsc::channel();

//     // "move" is used before the parameters list of the closure to move tx
//     // into the closure so the spawned thread ownes tx. The spawned thread 
//     // needs to own the transmitting end of the channel to send messages
//     // through it. Otherwise rust infers that it should borrow them. 
//     //
//     thread::spawn(move || {
//         let val = String::from("hi");
//         tx.send(val).unwrap();
//         // building more...
//         println!("val is {}", val);
//     });
//
//     // send() takes val and returns a Result<T, E> type
//     // unwrap() is being used to panic in case of an error
//     //
//     let received = rx.recv().unwrap();
//     println!("Got: {}", received);
// }

// // recv blocks the main threads execution and will wait until
// // a value is sent down the channel. 

// // now let's play with channel ownership transference
// // 


use std::time::Duration;

// New let's build a new main() that better proves concurrent action
// 
// fn main() {
//     let (tx, rx) = mpsc::channel();

//     // "move" is used before the parameters list of the closure to move tx
//     // into the closure so the spawned thread ownes tx. The spawned thread 
//     // needs to own the transmitting end of the channel to send messages
//     // through it. Otherwise rust infers that it should borrow them. 
//     //
//     thread::spawn(move || {
//         let vals = vec![
//             String::from("hi"),
//             String::from("from"),
//             String::from("the"),
//             String::from("thread"),
//         ];


//         for val in vals {
//             tx.send(val).unwrap();
//             thread::sleep(Duration::from_millis(500));
//         }        
//     });
//     // send() takes val and returns a Result<T, E> type
//     // unwrap() is being used to panic in case of an error

//     for received in rx {
//         println!("Got: {}", received);
//     }
// }



// Now improve further with a new sending handle, by cloning the transmitter. 
// Leave a single consumer (mpsc) and see how things go.
// 
fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    // First transmitter, but specify it's a clone of tx1 w/ref
    //
    thread::spawn(move || {
        let vals = vec![
            String::from("1. hi"),
            String::from("1. from"),
            String::from("1. the"),
            String::from("1. thread"),
        ];


        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(750));
        }        
    });

    // Second thread for original transmitter, also uses move
    thread::spawn(move || {
        let vals = vec![
            String::from("0. more"),
            String::from("0. messages"),
            String::from("0. for"),
            String::from("0. you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(750));
        }        
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
