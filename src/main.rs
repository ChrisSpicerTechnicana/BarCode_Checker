
use std::io;

fn calculate_control_digit(barcode : &[u32]) -> u32 {
    let sum_digits_in_odd_positions: u32 = barcode.iter().step_by(2).sum();
    println!("Sum of every other element: {sum_digits_in_odd_positions}");

    let sum_digits_in_even_positions : u32 = barcode.iter().skip(1).step_by(2).sum();
    let sum_digits_in_even_positions_times_3 : u32 = 3 * sum_digits_in_even_positions;
    println!("Sum of elements in even positions: {sum_digits_in_even_positions_times_3}");

    let sum_previous_results = sum_digits_in_odd_positions + sum_digits_in_even_positions_times_3;
    println!("Sum of the two previous results: {sum_previous_results}");

    let remainder = sum_previous_results % 10;

    let control_digit = (|remainder:u32| {
        if remainder == 0 {
            0
        }
        else {
            10 - remainder
        }
    })(remainder);

    return control_digit;
}

fn main() {

    println!("Enter a 12-digit barcode:");

    let mut guess: String = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess = guess.trim();

    const REQUIRED_BARCODE_LENGTH: usize = 12;
    if guess.len() != REQUIRED_BARCODE_LENGTH {
        panic!("Expected a 12-character barcode")
    }

    let mut barcode  = [0u32; REQUIRED_BARCODE_LENGTH];

    for index in 0..REQUIRED_BARCODE_LENGTH {
        let digit = guess
            .chars()
            .nth(index)
            .and_then(|ch| ch.to_digit(10));

        barcode[index] = digit.expect("Character at {index} is not numerical");
    }

    let control_digit: u32 = calculate_control_digit(&barcode);

    println!("Control Digit is: {control_digit}");
}
 