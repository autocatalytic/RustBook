use std::thread;
use std::time::Duration;
// fn main() {
//     thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
    
//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }
// }


// // And now running with the thread::spawn return type of JoinHandle
// fn main() {
//     let handle = thread::spawn(|| {
//         for i in 1..10 {
//             println!("hi number {} from the spawned thread!", i);
//             thread::sleep(Duration::from_millis(1));
//         }
//     });
    
//     for i in 1..5 {
//         println!("hi number {} from the main thread!", i);
//         thread::sleep(Duration::from_millis(1));
//     }

//     handle.join().unwrap(); // unwrap returns the contained Some value
//     // JoinHandle is an owned value that, when calling join() on it, 
//     // will wait for its thread to finish. In this case calling join()
//     // blocks the thead currently running until the handle terminates
// }


// here we run join() before the main thread
fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    handle.join().unwrap(); // join() blocks the main thread...

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// the move closure is often used alongside thread::spawn to move
// data from one thread to another thread. This is especially useful
// to transfer ownership of values from one thread to another

