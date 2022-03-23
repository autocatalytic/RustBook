use blog::Post;

// Implementing an Object-Oriented Design Pattern
//
// 
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today.");
//    assert_eq!("", post.content());
//
//    post.request_review();
//    assert_eq!("", post.content());
//    post.approve();

    // our changes to make main reassign post mean our implementation
    // doesn't follow object-oriented state pattern anymore. The benefit
    // is that invalid states are now impossible because of the rust
    // type system and type checking at compile time.
    //
    let post = post.request_review();

    let post = post.approve();

    assert_eq!("I ate a salad for lunch today.", post.content());
}
