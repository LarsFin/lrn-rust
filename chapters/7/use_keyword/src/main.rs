mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("added to waitlist!");
        }
    }
}

mod back_of_house {
    pub mod kitchen {
        pub fn prepare_food() {
            println!("prepared food!");
        }
    }
}

mod customer {
    use super::front_of_house::hosting;

    // you can use the 'use' keyword to directly bring the function into scope
    // but it has the downside of making it unclear where the function is declared
    // however, it's idiomatic to use the full path for enums and structs
    use super::back_of_house::kitchen::prepare_food;

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }

    pub fn order_food() {
        prepare_food();
    }
}

// similar to other programming languages you can use 'as' to alias a module member,
// this helps distinguish between two members from two different modules with the same
// name
mod strings;
mod integers;

use strings::add as add_strings;
use integers::add as add_integers;

use rand::Rng;

// importing multiple modules from the same crate can be simplified
// use std::cmp::Ordering;
// use std::io;
// becomes 'use std::{cmp::Ordering, io};'

// this can also be done with importing parent and child modules
// use std::io;
// use std::io::Write;
// becomes 'use std::io::{self, Write};'

// you can also use the glob operator to import all public items from a module
// use std::collections::*;

fn main() {
    customer::eat_at_restaurant();
    customer::order_food();

    println!("Adding 1 and 2: {}", add_integers(1, 2));
    println!(
        "Adding 'hello ' and 'world!': {}",
        add_strings(&String::from("hello "), &String::from("world!"))
    );

    let secret_number = rand::thread_rng().gen_range(1..101); // or 1..=100
    println!("The secret number is: {}", secret_number);
}
