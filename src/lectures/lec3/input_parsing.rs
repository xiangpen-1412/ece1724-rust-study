// Lecture 3, slide 9: input text -> trim -> parse -> number, using shadowing between stages.
// Run target: lec3_input_parsing
// Enter a line of input in the run window and press Enter.

use std::io;

fn main() {
    // The original guess is a mutable String that receives a line of input.
    let mut guess = String::new();
    println!("请输入一个非负整数（例如 42），然后按 Enter：");

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read input");

    // &mut guess allows read_line to append input to this string.
    // Input usually includes a trailing newline; trim returns a string slice without
    // leading or trailing whitespace, leaving the original String unchanged.
    // u32 specifies the target integer type for parse.
    let guess: u32 = guess
        .trim()
        .parse()
        .expect("Expected a non-negative integer within the u32 range");

    // The new guess is an immutable u32; it does not inherit the original binding's mutability.
    println!("Parsed number: {guess}");

    // Try entering "42", "  42  " with surrounding spaces, "abc", and "-1".
    // The last two fail to parse; expect panics and terminates this simple program.
    // The expect message reports failure; it is not an input prompt and does not retry.
}
