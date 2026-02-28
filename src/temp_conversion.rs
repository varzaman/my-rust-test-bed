use std::io;

pub fn temperature_conversion() {
    println!("Convert temperature between Celsius and Farenheit.");
    println!("!c followed by a number will convert the number from farenheit to celsius.");
    println!("!f followed by a number will convert the number from celsius to farenheit.");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Please enter a valid selection.");

    let split_input: Vec<&str> = input.trim().split_whitespace().collect();

    if split_input.len() >= 2 {
        let command = split_input[0];
        let value = split_input[1];

        let temp_input: f64 = match value.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid temperature as an integer or decimal.");
                0.0
            }
        };

        match command {
            "!f" => println!(
                "The converted value is {:.2} degrees Farenheit",
                convert_to_farenheit(temp_input)
            ),
            "!c" => println!(
                "The converted value is {:.2} degrees Celsius",
                convert_to_celsius(temp_input)
            ),
            _ => println!("{} is not a valid conversion", command),
        }
    }
}

fn convert_to_celsius(farenheit: f64) -> f64 {
    (farenheit - 32.0) * (5.0 / 9.0)
}

fn convert_to_farenheit(celsius: f64) -> f64 {
    (celsius * (9.0 / 5.0)) + 32.0
}
