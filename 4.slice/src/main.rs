fn main() {
   let mut s = String::from("Hello world");

   let word = first_word(&s); // word will get the value 5

//   s.clear(); // this empties the string, making it equal to ""

   // word still has value 5 here, but there's no more string that
   // we could meaningfully use the value 5 with. word is now totally invalid!!
   println!("S is: {} and WORD is: {:?}", s, word);
}

fn first_word(s: &String) -> &str {
    // don't need a delimiter, as we're using space
    // 
    // do we have access to string parsing features?
    //let word = substr(&s, " ");
    // 
    // Okay, let's do it from the basics

    // let bytes = s.as_bytes();

    // for (i, &item) in bytes.iter().enumerate() {
    //     if item == b' ' {
    //         return i;
    //     }
    // }

    // // Ugh, that could get ugly. All that work and we only have an index
    // // and size that could be unrelated to the original context (input string)

    // // Try Again, with SLICE!

    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
