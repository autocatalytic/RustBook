use std::str::Chars;

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    //let mut largest = number_list[0];

    // for number in number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }

    // println!("The largest number is {}", largest);

    // now, using our extracted function
    let result = largest(&number_list);
    println!("Largest using abstracted function is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

    // now, using our abtracted function instead of above
    let result = largest(&number_list);
    println!("Largest using new list and abstracted function is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let char_result = largest_char(&char_list);
    println!("Largest char from abstracted function is {}", char_result);
    
    // utilize the method on Point
    let p = Point { x: 5, y: 10 };
    println!("Demonstrate the 'x()' method on the point Type p.x = {}", p.x());

    // But generics require both types in a function to be the same!
    //
    // so this will break because y isn't an integer:
    //
    let wont_work = Point { x: 5, y: 4.0 };
    //
    // until we fix the struct to have two types.


    // 
    // Working with traits and methods, using the structs below.
    //
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people"
        ),
        reply: false,
        retweet: false,
    };
    
    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Giants Win World Series!"),
        location: String::from("San Francisco, CA"),
        author: String::from("C.W. Nevius"),
        content: String::from("For a while they did it every odd year!"),
    };
    
    println!("Summary: {}", article.summarize());

}

// This function is pulled from the main() body above, for better organization
//
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

// Same here, and notice how similar this is to the function above
// 
fn largest_char(list: &[char]) -> char {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Now let's eliminate duplication by introducing generic Type parameters
// 
// This won't compile yet, because not all types can be compared 
// with greater than
//
//fn gLargest<T>(list: &[T]) -> T {
//    let mut largest = list[0];
//
//    for &item in list {
//        if item > largest {
//            largest = item;
//        }
//    }
//
//    largest
//}


// Now let's look at using generic type parameters in structs
// 
struct Point<T, U> {
    x: T,
    y: U,
}


impl<T, U> Point<T, U> { 
// Note that we have to declare T just after impl to specify we're implementing
// methods on the type Point<T>. By declaring T as a generic type after impl,
// Rust can identify that the type in the angle brackets in Point is a 
// generic type rather than a concrete type.
// 
    fn x(&self) -> &T {
        &self.x
    }
}

// Rust compiler uses monomorphization of the code using generics, so code
// written this way performs as well as code with concrete types. It does this
// by working back from the generics to check where they are used. Wow!


pub trait Summary {
    fn summarize(&self) -> String;
}

// Playing with defining and using traits
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
