// Blog post workflow using object-oriented design pattern: State Pattern

// State pattern: a value has some internal state, which is represented by
// a set of state objects, and the value's behavior changes based on the
// internal state. The 'state objects' share functionality, and in Rust
// we use structs and traits to represent this sharing

// Using the state pattern means when business requirements of the program
// change, we won't need to change the code of the value holding the state
// or the code that uses the value. 
//
// This strikes me as different from the use in games or blockchains
// where the "state" reflects memorization of data and conditions. In this 
// definition here, it's more of an API (or integration) state. Although
// in this assignment the state refers to whether our posts have been
// published or not, so maybe it is the memory/condition idea.
//

pub struct Post {
    state: Option<Box<dyn State>>,  // Option because using take() below,
    content: String,                // which sets this to empty at first
}

impl Post {
    pub fn new() -> Post {
        // A post starts in the draft state, private ensures it
        Post {
            state: Some(Box::new(Draft {})), 
            content: String::new(),
        }
    }

    // &mut self because changing Post where calling add_text on
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }
    
    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

}

// defines behavior shared by different post states, and the Draft
trait State {
    // all types that implement State trait will now need to implement
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;

    // default impl for content() returns an empty string slice,
    // which means we don't need to implement content on Draft and 
    // PendingReview structs. Published struct will override content()
    fn content<'a>(&self, post: &'a Post) -> &'a str {
        ""
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        Box::new(Published {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}
