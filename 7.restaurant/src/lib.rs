mod front_of_house;

use crate::front_of_house::hosting;

mod back_of_house {
    use crate::front_of_house;

    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub struct Breakfast {
        pub toast: String,
        pub seasonal_fruit: String, 
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        // No longer need super, moved to mod at top
        // super::serve_order();
        front_of_house::serving::serve_order();
    }

    fn cook_order() {}
}

// don't this we need this anymore, importing at top
// use self:front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");

    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    meal.seasonal_fruit = String::from("blueberries");
    println!("With my {} on the side.", meal.seasonal_fruit);

    let order1= back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

}
