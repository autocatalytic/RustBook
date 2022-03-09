// Methods are defined within the context of a struct (or enum or trait)
// and their first parameter is always "self", which represents the instance
// of the struct the method is being called on.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// New definition, impl block. It's used to implement a struct
// there can be many for one struct
impl Rectangle {
    fn area(&self) -> u32 {
        // self is 1st parameter, required to make method called area
        // for the struct Rectangle.
        self.width * self.height
    }

    fn can_hold(&self , other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // Associated function. Lets you namespace functionality that is
    // particular to your struct without having an instance available.
    // In this case, get the area of a square using only one input,
    // but by using the associated area function.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}


fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2= Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!(
        "The area of the rectangle 1 is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2?: {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3?: {}", rect1.can_hold(&rect3));

    // Let's use our associated function
    println!("What's a square with width of rect1 ({})?: {:?}", rect1.width,
    Rectangle::square(rect1.width).area());     // this is cool.
}
