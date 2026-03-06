use std::io;

use crate::menu::inline_prompt;
use crate::menu::render_menu;

pub fn string_stuff() {
    let menu_items = ["Get the first word of a sentence."];
    let menu_selection = render_menu(&menu_items);

    match menu_selection {
        0 => first_word_menu(),
        _ => println!("Please enter a valid selection."),
    }
}

fn first_word_menu() {
    inline_prompt("Please enter your sentence: ");

    let mut sentence = String::new();

    io::stdin()
        .read_line(&mut sentence)
        .expect("Failed to read line");

    let first_word = get_first_word(&sentence);

    println!("The first word is: {}", first_word);
}

fn get_first_word(sentence: &String) -> String {
    let chars = sentence.chars();

    let mut first_word = String::new();
    for char in chars {
        if char == ' ' {
            break;
        }
        first_word.push(char);
    }

    first_word
}
