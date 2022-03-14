// Lifetimes in Rust keep track of when references come into and ouf of scope
//
// Items that have gone out of scope have their memory deallocated, and
// preventing references to invalid memory is much more safe
//
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is: {}", result);

    let string3 = String::from("long string is long");
    let result2;
    {
        let string4 = String::from("xyz");
        result2 = longest(string3.as_str(), string4.as_str());
        println!("The longest string is {}", result2);
    }
    // move the print here and result is out of scope, an invalid lifetime
    // println!("The longest string is {}", result2);


    // Lifetime annotations in Struct Definitions
    //
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}

// Writing this function without annotating lifetimes will not compile
// so we replace "&str" with "&a' str" to specify x and y should have the
// same lifetime. a' is the traditional generic lifetime parameter, but it 
// can be anoher character.
//
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Lifetime specifications are required for structs that hold references, 
// for each reference in the struct's definition. In this case, the data
// in novel exists before ImportantExcerpt is created and does not go out
// of scope until after ImportantExcerpt goes outof scope, but without
// lifetime annotation the compiler would not know that.
struct ImportantExcerpt<'a> {
    part: &'a str,
}

// I must admit that a lot of the discussion on the history of lifetime
// parameters and signature inferrence went over my head. Hopefully I 
// can make it more concrete by writing more Rust code.