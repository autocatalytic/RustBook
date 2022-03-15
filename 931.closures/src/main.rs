use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;

// Found solution: https://gist.github.com/821760408-sp/2b36a71d79cea977eef73c5249d477f0
// never would have figured it out alone!! But makes way more sense.
// 
fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout(
        simulated_user_specified_value, 
        simulated_random_number
    );
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println! (
                "Today, run for {} minutes!", 
                expensive_result.value(intensity)
            );
        }
    }

}

struct Cacher<T, U, V>
    where U: Eq + Hash + Copy, V: Clone, T: Fn(U) -> V
{
    calculation: T, 
    value: HashMap<U, V>,
}

impl<T, U, V> Cacher<T, U, V>
where U: Eq + Hash + Copy, V: Clone, T: Fn(U) -> V
{
    fn new(calculation: T) -> Cacher<T, U, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: U) -> V {
        let v = self.value.entry(arg).or_insert((self.calculation)(arg));
        
        v.clone()
    }

}

#[test]
fn call_with_different_values() {
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}