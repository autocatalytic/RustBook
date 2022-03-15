use std::collections::HashMap;
// Functional language features: Iterators and Closures
// 
use std::thread;
use std::time::Duration;

// Closures are anonymous functions that can capture their environment
// and which you can store in a variable
// 
// So we can pull this function into the generate_workout function using
// the closure structure
//
//fn simulated_expensive_calculation(intensity: u32) -> u32 {
//    println!("calculating slowly...");
//    thread::sleep(Duration::from_secs(3));
//    intensity
//}


fn main() {
    let simulated_user_specified_value = 25; // Does the user want to work hard?
    let simulated_random_number = 7;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

fn generate_workout(intensity: u32, random_number: u32) {
    // Here we're only calling the expesive function once, but still calling
    // it even if we don't need it, as when intensity > 25 and we take a break
        //let expensive_result = simulated_expensive_calculation(intensity);
    // 
    // Instead let's use a closure that will store the definition of our
    // expensive function, not the resulting value of calling it, and then
    // create a struct that will only call the function if it has not already
    // been called. Cool!
    // 
    // Since closures are not exposed to users as part of the library, they
    // do not require type annotations like fn functions do. They are optional.
    // 
    let mut expensive_closure = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(3));
        num
    });
    

    if intensity < 25 {
        println!( "Today, do {} pushups!", expensive_closure.value(intensity));
        println!( "Next, do {} situps!", expensive_closure.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_closure.value(intensity));
        }
    }
}

// Using structs to cache values from closures is called memoization or lazy
// evaluation in other contexts
//
struct Cacher<T> 
where 
    T: Fn(u32) -> u32,      // Trait bounds specify it's a closure by using Fn
{
    calculation: T,
    value: Option<u32>,     // Refactor to HashMap storing multiple calcs. Was:
                            // Option, so we can grab or calculate as needed!
}

impl<T> Cacher<T>
where
    T: Fn(u32) ->  u32,
{
    // return new cacher instance with the closure "calculation"
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    // the "calling code" uses the value method, which calculates if necessary
    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);    // accepts user supplied val
                self.value = Some(v);
                v
            }
        }
    }
}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}

