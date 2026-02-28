pub fn twelve_days_of_christmas() {
    let lines = [
        "A partridge in a pear tree.",
        "Two turtle doves,",
        "Three French hens,",
        "Four calling birds",
        "Five gold rings,",
        "Six geese a-laying,",
        "Seven swans a-swimming,",
        "Eight maids a-milking,",
        "Nine ladies dancing,",
        "Ten lords a-leaping,",
        "Eleven pipers piping,",
        "Twelve drummers drumming,",
    ];

    let alt_first_line = "And a partridge in a pear tree.";

    for day in 0..12 {
        day_of_christmas(day);
        println!("{}", lines[day]);

        for prev_day in (0..day).rev() {
            if prev_day > 0 {
                println!("{}", lines[prev_day]);
            } else {
                println!("{}", alt_first_line);
            }
        }
        println!("\n");
    }
}

fn day_of_christmas(index: usize) {
    let days = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelth",
    ];

    println!(
        "On the {} of Christmas my true love sent to me",
        days[index]
    )
}
