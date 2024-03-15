use back_of_house::Breakfast;

mod bathroom;

mod front_of_house {
    pub fn add_to_waitlist() {
        println!("added to waitlist");
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    pub mod chef {
        pub fn cook_breakfast() -> super::Breakfast {
            super::Breakfast {
                toast: String::from("bread"),
                seasonal_fruit: String::from("apple"),
            }
        }

        pub fn throw_away_breakfast(breakfast: &super::Breakfast) {
            println!("threw away {} and {}", breakfast.toast, breakfast.seasonal_fruit);
        }
    }
}

pub fn eat_breakfast(breakfast: &Breakfast) {
    println!("eating toasted {}", breakfast.toast);

    // seasonal_fruit is private to the back_of_house module so will error
    // println!("Eating seasonal fruit {}", breakfast.seasonal_fruit);
}

fn main() {
    // absolute path
    crate::front_of_house::add_to_waitlist();

    // relative path
    front_of_house::add_to_waitlist();

    bathroom::cleaner::work();

    let breakfast = back_of_house::chef::cook_breakfast();

    eat_breakfast(&breakfast);

    back_of_house::chef::throw_away_breakfast(&breakfast);
}
