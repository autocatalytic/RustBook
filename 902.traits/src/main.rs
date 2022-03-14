mod lib;
use crate::lib::*;

// Traits tell the Rust compiler about funcationality types can 
// share with other types. Trait bounds specify that generic types can 
// be any type that has a certain behavior.
// 
// In this example, we have structs for two types of written media and need 
// a summary for each type.
// 
fn main() {
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

    // Traits as Parameters
    // 
    // Call the summarize on its item parameter, using the impl Trait syntax
    // pub fn notify(item: &impl Summary) {
    //     println!("Breaking news! {}", item.summarize());
    // }

    // 
    // But this is the longer form of "Trait Bound Syntax". The "impl Trait"
    // syntax above is convenient and concise for simple cases.
    //
    pub fn notify<T: Summary>(item: &T) {
        println!("Breaking news! {}", item.summarize());
    }

    notify(&article);


    // Returning types that Implement Traits
    //
    // we can also use "imple Trait" syntax in the return Traits
    //
    fn returns_summarizable() -> impl Summary {
        Tweet {
            username: String::from("amazon_ebooks"),
            content: String::from(
                "of course, cheaper, faster and simpler, people",
            ),
            reply: false,
            retweet: false,
        }
    }

    // However you can only use imple Trait if you're returning a single type
    // 
    // This won't work because of restrictions around how impl trait is 
    // implemented in the compiler
    // 
//    fn returns_summarizable(switch: bool) -> impl Summary {
//        if switch {
//            NewsArticle {
//                headline: String::from(
//                    "Penguins win the Stanley Cup Championship!",
//                ),
//                location: String::from("Pittsburgh, PA, USA"),
//                author: String::from("Iceburgh"),
//                content: String::from(
//                    "The Pittsburgh Penguins once again are the best \
//                     hockey team in the NHL.",
//                ),
//            }
//        } else {
//           Tweet {
//               username: String::from("amazon_ebooks"),
//               content: String::from(
//                   "of course, cheaper, faster and simpler, people",
//               ),
//               reply: false,
//               retweet: false,
//           }
//       }
//    }


    // Going back to the "largest" function in generics chapter
    //
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("Largest using function is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("Largest using function is {}", result);

    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];
    
        for &item in list {
            if item > largest {
                largest = item;
            }
        }
    
        largest
    }

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

}

// Skipping "using trait bounds to conditionally implement methods"
fn come_back_later() {
    todo!(); 
}
