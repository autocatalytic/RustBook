fn main() {
    let data = "initial contents";

    let szero = data.to_string();

    println!("String zero: {}", szero);

    // the method also works on a literal directly:
    let s = "initial contents".to_string();

    println!("String: {}", s);

    let s2 = String::from("secondary contents");

    println!("Second string: {}", s2);

    let hello = String::from("السلام عليكم");

    println!("{}", hello);

    let s3 = String::from("tic");
    let s4 = String::from("tac");
    let s5 = String::from("toe");

    let game = format!("{}-{}-{}", s3, s4, s5);

    println!("Favorite game: {}", game);

    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }

}


