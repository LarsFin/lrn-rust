fn main() {
    let penny = Coin::Penny;
    let nickel = Coin::Nickel;
    let dime = Coin::Dime;
    let quarter = Coin::Quarter;

    println!("Value of penny: {} cents", value_of_coin(&penny));
    println!("Value of nickel: {} cents", value_of_coin(&nickel));
    println!("Value of dime: {} cents", value_of_coin(&dime));
    println!("Value of quarter: {} cents", value_of_coin(&quarter));

    let coins = [penny, nickel, dime, quarter];
    println!("Total value of coins: {} cents", value_of_coins(&coins));

    let penny_1 = Coin::Penny;
    let penny_2 = Coin::Penny;
    let penny_3 = Coin::Penny;
    let nickel_1 = Coin::Nickel;
    let dime_1 = Coin::Dime;
    let dime_2 = Coin::Dime;

    let more_coins = [penny_1, penny_2, penny_3, nickel_1, dime_1, dime_2];

    println!("Number of pennies: {}", count_coins_by_type(&more_coins, Coin::Penny));
    println!("Number of nickels: {}", count_coins_by_type(&more_coins, Coin::Nickel));
    println!("Number of dimes: {}", count_coins_by_type(&more_coins, Coin::Dime));
    println!("Number of quarters: {}", count_coins_by_type(&more_coins, Coin::Quarter));

    println!("Only pennies: {}", only_pennies(&more_coins));
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_of_coin(coin: &Coin) -> u8 {
    // rust's match expression is exhaustive, so we must handle all possible cases
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn value_of_coins(coins: &[Coin]) -> u8 {
    let mut total = 0;
    for coin in coins {
        total += value_of_coin(coin);
    }
    total
}

fn only_pennies(coins: &[Coin]) -> bool {
    for coin in coins {
        // we can use a variable name to match any value, rust also uses the _ prefix to denote
        // we wont use the variable in the arm
        match coin {
            Coin::Penny => {
                continue;
            }
            _ => {
                return false;
            }
        }
    }
    false
}

fn count_coins_by_type(coins: &[Coin], coin_type: Coin) -> u8 {
    let mut count = 0;
    for coin in coins {
        match coin_type {
            Coin::Penny => {
                if let Coin::Penny = coin {
                    count += 1;
                }
            }
            Coin::Nickel => {
                if let Coin::Nickel = coin {
                    count += 1;
                }
            }
            Coin::Dime => {
                if let Coin::Dime = coin {
                    count += 1;
                }
            }
            Coin::Quarter => {
                if let Coin::Quarter = coin {
                    count += 1;
                }
            }
        }
    }
    count
}
