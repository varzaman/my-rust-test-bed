use std::io;

mod fibonacci_sequence;
mod guessing_game;
mod temp_conversion;
mod twelve_days_christmas;

use fibonacci_sequence::fibonacci_sequence;
use guessing_game::guessing_game;
use temp_conversion::temperature_conversion;
use twelve_days_christmas::twelve_days_of_christmas;

fn print_option(opt: &str) {
    println!("{}", opt)
}

fn main() {
    print_option("1. Print the Fibonacci sequence");
    print_option("2. Print the lyrics to the Twelve Days of Christmas");
    print_option("3. Farenheit to Celsius and vice versa convertor");
    print_option("4. The Guessing Game");

    let mut selection = String::new();

    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read line");

    let selection: u32 = match selection.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Please enter a valid selection.");
            0
        }
    };

    match selection {
        1 => fibonacci_sequence(10),
        2 => twelve_days_of_christmas(),
        3 => temperature_conversion(),
        4 => guessing_game(),
        _ => println!("Please enter a valid selection."),
    }
}
