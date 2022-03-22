
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

