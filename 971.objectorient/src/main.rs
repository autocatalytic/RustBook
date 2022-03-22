
use objectorient::{Draw, Button, Screen, AveragedCollection};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // code to actually draw select box
    }
}

fn main() {

    let mut collection: AveragedCollection = AveragedCollection {
        list: vec!{1, 2, 3, 5},
        average: 1.1,
    };

    collection.add(7);
//    collection.remove(); 

    println!("collection average: {:?}", collection.average);

//    let screen = Screen {
//        components: vec![Box::new(String::from("Hi"))],
//    };
//
//    screen.run();
}
