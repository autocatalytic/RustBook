// Collections are sets of values or data. There are three main types:
//  o Vectors store a variable number of values next to eachother
//  o Strings are collections of characters
//  o Hash Maps allow key:value storage, and is part of "map" structures

fn main() {

    let v = vec![1, 2, 3, 4, 5];

    let third = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element.")
    }

    let mut v2 = v;
    for i in &mut v2 {
        *i += 50;
        println!("{}", i);
    }

    println!("Element 2 of the row is: {}", v2[1]);

    // The vec type will not let you store different types in the same vector
    // but enum will! 
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let mut vec1 = vec![1, 2, 3, 4];
    let vec2 = vec![10, 20, 30, 40];

    println!("vector 1: {:?}", vec1);

    // Playing with iter
    // the extend method will call into_iter to append one vector
    // to another.
    vec1.extend(vec2);

    for i in vec1 {
        println!("Appended vec1 item: {:?}", i);
    }

    // appending vec2 with vec1 will not work though, since vec2 
    // is not mutable.


}
