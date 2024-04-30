#![allow(unused)]

mod front_of_house;

// use statements link together things in the file system via a "shortcut"
// functions: bring function's parent module into scope
use crate::front_of_house::waiting_area;
// structs, enums, and other items: specify full path
use std::collections::HashMap;
// can provide aliases to avoid naming conflicts
use std::fmt::Result;
use std::io::Result as IoResult;
// nested paths can clean up large use lists
// use std::cmp::Ordering;
// use std::io;
// ...becomes...
use std::{cmp::Ordering, io};
// glob operator * brings in all public items- use with caution!
use std::collections::*;

pub fn eat_at_restaurant() {
    // Absolute path (generally preferred, more likely to move code defn's & item calls independently of each other)
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // Calls via "use" statement
    waiting_area::add_new_customer();
    waiting_area::take_customer_to_table();
    let mut map = HashMap::new();
    map.insert(1, 2);

    let mut meal = back_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}

mod back_of_house {
    // pub enum variants are public by default
    pub enum Appetizer {
        Soup,
        Salad,
    }

    // pub struct fields are private by default
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

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
