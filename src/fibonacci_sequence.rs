// Prints the fibonacci sequence for the specified number of numbers.
pub fn fibonacci_sequence(n: u32) {
    print!("0, 1");

    let mut prev_numbers: [u32; 2] = [0, 1];
    for _i in 2..n {
        let sum = prev_numbers[0] + prev_numbers[1];

        prev_numbers[0] = prev_numbers[1];
        prev_numbers[1] = sum;

        print!(", {}", sum)
    }
}
