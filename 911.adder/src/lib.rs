#[cfg(test)]
mod tests {
    use super::*; // tests is an inner module, need to bring in outer modules

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    // fn another() {
    //     panic!("Make this test fail");
    // }

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }

    // Another way to write this test, using Ok and Err macros, and Result types
    // instead of the assert_eq! macro. 
    //
    // Writing rests so they return a Result<T, E> enables you to use the 
    // question mark operator in the body of tests, which is a convenient way
    // to write tests that should fail if any operation within them returns
    // an Err variant.
    // 
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        //let result = greeting("Phoebe");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
    );
    }

    #[test]
    #[ignore]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }

}

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
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub fn greeting(name: &str) -> String {
    format!("Hello {}!", name)
//    format!("Hello!")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 { // || value > 100 {
            panic!(
                "Guess value must be gte 1, got {}.", 
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.", 
                value
            );
            }

        Guess { value }
    }
}
