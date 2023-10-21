use garden::vegetables::Asparagus;

use crate::front_of_house::hosting;

pub mod garden;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    #[derive(Debug)]
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    pub fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative
    front_of_house::hosting::add_to_waitlist();

    // use
    hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("rye");
    meal.toast = String::from("wheat");
    println!("I'd like {} toast please", meal.toast);

    let _order1 = back_of_house::Appetizer::Soup;
    let _order2 = back_of_house::Appetizer::Salad;
}

fn main() {
    let plant = Asparagus {};
    println!("This is asparagus: {:?}", plant);

    back_of_house::fix_incorrect_order();
}
