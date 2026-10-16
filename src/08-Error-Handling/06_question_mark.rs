// Lecture 6 - Error Handling: The ? operator
// Run: cargo run --bin errors_question_mark
// TODO: Write a function returning Result and propagate file errors with ?. Call it from main. You can also try making main return Result.

use std::fs::File;
use std::io::{Error, Read};

fn read_username_from_file_question_mark() -> Result<String, Error> {
    let mut username = String::new();
    File::open("llms.txt")?.read_to_string(&mut username)?;
    Ok(username)
}


fn main() {
    let sample_path = "src/08-Error-Handling/data/hello.txt";
    let missing_path = "src/08-Error-Handling/data/missing.txt";
    println!("Sample path: {sample_path}");
    println!("Missing path: {missing_path}");

    // Write your code here.
}
