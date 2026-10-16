// Lecture 6 - Error Handling: Result, Ok, and Err
// Run: cargo run --bin errors_result
// TODO: Create Ok and Err values here. Choose their success and error types.

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    // Write your code here.
    let greeting_file_result = File::open("llms.txt");

    match greeting_file_result {
        Ok(_) => { println!("WOW"); },
        Err(err) => match err.kind() {
            ErrorKind::NotFound => {
                File::create("/src/08-Error-Handling/llms.txt").expect("Create LLMs failed");
            },
            _ => { println!("error: {:?}", err); },
        }
    }
}
