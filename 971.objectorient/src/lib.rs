// From first exercises around object oriented-style Rust code
// 
pub struct AveragedCollection {
    pub list: Vec<i32>, // adding pub, just to test
    pub average: f64,   // same, though it breaks the exercise goal ;-P
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();   // last vec value
        match result {
            // only removes last element of vec added
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}

// Now digging into how trait objects allow Rust to be polymorphic
//
// To mimic inheritence we  define a Draw trait containing a draw  method, 
// which allows us to draw each component value of the system to the screen. 
/*  To do this we will:

        - define a trait having one method: draw
        - define "trait objects" with Draw trait *and* draw method specifics
        - (note: these can be used in place of generic or concrete Types)
        - then we can define a vector that takes trait objects
        - and draw the entire vector of components to screen

*/

// Trait objects aren't as generally useful as objects in other languages:
// their specific purpose is to allow abstraction across common behavior.
//
pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>, // any type inside a Box that
}                                       // implements the Draw trait

// Whereas a generic type param can only be substituted with one concrete
// type at a time, trait objects allow for multiple concrete types to fill
// in for the trait object at runtime.
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// Note that using generics with trait bounds is more performant than
// trait objects, if you know your collections are homogenous

// Now let's implement some types of Draw traits. Note that a TextField
// type may have additional fields like placeholder, but implements Draw
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

