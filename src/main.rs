mod fibonacci_sequence;
mod guessing_game;
mod menu;
mod rectangles;
mod string_stuff;
mod temp_conversion;
mod twelve_days_christmas;

use fibonacci_sequence::fibonacci_sequence;
use guessing_game::guessing_game;
use menu::render_menu;
use rectangles::rectangles;
use string_stuff::string_stuff;
use temp_conversion::temperature_conversion;
use twelve_days_christmas::twelve_days_of_christmas;

fn main() {
    let menu_items = [
        "Print the Fibonacci sequence",
        "Print the lyrics to the Twelve Days of Christmas",
        "Farenheit to Celsius and vice versa convertor",
        "The Guessing Game",
        "String stuff",
        "Rectangles",
    ];
    let menu_selection = render_menu(&menu_items);
    match menu_selection {
        0 => fibonacci_sequence(10),
        1 => twelve_days_of_christmas(),
        2 => temperature_conversion(),
        3 => guessing_game(),
        4 => string_stuff(),
        5 => rectangles(),
        _ => println!("Please enter a valid selection."),
    }
}
