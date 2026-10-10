// Run: lec3_input_parsing

use std::io;

fn main() {
    let mut guess = String::new();
    println!("请输入一个非负整数（例如 42），然后按 Enter：");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    // Trim whitespace, parse u32, then shadow the String binding.
    let guess: u32 = guess
        .trim()
        .parse()
        .expect("Expected a non-negative integer within the u32 range");

    println!("Parsed number: {guess}");

    // Try "42", "  42  ", "abc", and "-1"; invalid input panics.
}
