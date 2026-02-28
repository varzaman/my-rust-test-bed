use rand::RngExt;
use std::cmp::Ordering;
use std::io;

pub fn guessing_game() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);

    loop {
        println!("Test your luck.Input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is {}.", guess);

        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Your guess is too large"),
            Ordering::Less => println!("Too small!"),
            Ordering::Equal => {
                println!("\u{1F3C6} You're winner!\u{1F3C6}");
                break;
            }
        }
    }
}
