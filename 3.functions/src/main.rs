

fn main() {
    
    for number in (1..five()).rev() {
        println!("{}!", number);

    }
    println!("LIFTOFF!!!");
}

fn five() -> usize {
    5
}

