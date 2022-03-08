// The point of ownership in rust is to manage heap data :-)
//
fn main() {
    let _s1 = gives_ownership();         // gives_ownership moves its return
                                                 // value into s1
    println!("Do I have _s1? {}", _s1);

    let s2 = String::from("hello");     // s2 comes into scope

    let _s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
    println!("Do I have _s3? {}", _s3);

 } // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
   // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {
    let some_string = String::from("hello");

    some_string
}

// take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into scope
                                                      // 

    a_string  // a_string is returned and moves out to the calling function
} 
