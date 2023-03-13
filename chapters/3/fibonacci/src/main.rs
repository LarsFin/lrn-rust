use std::io;

fn main() {
    println!("Enter the iterative of the fibonacci sequence");
    let input = get_input();
    let fibonacci_value = fibonacci(input);
    println!("Fibonacci of {input} is {fibonacci_value}");
}

fn get_input() -> u32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().parse().expect("Please type a number!")
}

fn fibonacci(n: u32) -> u32 {
    let mut value: u32 = 0;
    let mut i: u32 = 1;

    while i <= n {
        value += i;
        i += 1;
    }

    return value;
}