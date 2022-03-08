// fn main() {
//     let s1 = String::from("hello");

//     let (s2, len) = calculate_length(s1);

//     println!("The length of '{}' is {}.", s2, len);

// }

// fn calculate_length(s: String) -> (String, usize) {
//     let length = s.len();

//     (s, length)
// }

// Now run the above without transferring ownership of the value
fn main() {
    let s1 = String::from("hello");

    // here, the &s1 syntax refers to s1 but does not own it
    // this means the value it points to will not be dropped when
    // the reference goes out of scope
    let len = calculate_length(&s1);

     println!("The length of '{}' is {}.", s1, len);
}

// Notice &String instead of String. Apersands are references, and they allow you
// to refer to some value without taking ownership of it. 
fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope, but because it does not have ownership of 
  // what it refers to, nothing happens.

// Note: a dereference operator is *

// In rust having references as function parameters are called borrowing.

