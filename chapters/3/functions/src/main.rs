use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = gen_secret();

    loop {
        println!("Please input a number");

        let guess = get_input();

        if is_exit(&guess) {
            break;
        }

        let (is_valid, guess) = parse_input(&guess);

        if !is_valid {
            println!("Make sure to input a number!");
            continue;
        }

        let (is_winner, message) = compare_guess(&secret_number, &guess);

        println!("{message}");

        if is_winner {
            break;
        }
    }
}

fn gen_secret() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}

fn get_input() -> String {
    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    guess
}

fn is_exit(guess: &String) -> bool {
    guess.contains("exit")
}

fn parse_input(guess: &String) -> (bool, u32) {
    match guess.trim().parse() {
        Ok(num) => (true, num),
        Err(_) => (false, 0),
    }
}

fn compare_guess(secret: &u32, guess: &u32) -> (bool, String) {
    match guess.cmp(&secret) {
        Ordering::Less => (false, "Too small!".to_string()),
        Ordering::Greater => (false, "Too big!".to_string()),
        Ordering::Equal => (true, "You win!".to_string()),
    }
}