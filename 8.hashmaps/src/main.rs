fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // now update the hashmap
    //
    scores.insert(String::from("Blue"), 20);

    // only updates the value matching the key
    //
    println!("{:?}", scores);

    scores.entry(String::from("Green")).or_insert(30);

    println!("{:?}", scores);

    // Updating a hashmap based on an old value
    //
    let text = "hello world wonderful world wonderful hello world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);  // or_insert returns mut reference
        *count += 1;   // must dereference count before storing the value
    }

    println!("{:?}", map);

}

// By default HashMa uses SipHash that provides resistance to DoS attacks
// but it is not the fastest. Others are available with parameter: hasher
// See crates.io libraries for other hasher algorithms.