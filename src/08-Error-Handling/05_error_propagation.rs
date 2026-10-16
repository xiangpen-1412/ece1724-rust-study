// Lecture 6 - Error Handling: Returning errors to the caller
// Run: cargo run --bin errors_propagation
// TODO: Write a function that reads the sample file and returns Result<String, std::io::Error>. Use match to return errors to the caller.

use std::fs::File;
use std::io::{Error, Read};

fn read_username_from_file() -> Result<String, Error> {
    let username_file_result = File::open("llms.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn main() {
    let sample_path = "src/08-Error-Handling/data/hello.txt";
    let missing_path = "src/08-Error-Handling/data/missing.txt";
    println!("Sample path: {sample_path}");
    println!("Missing path: {missing_path}");

    // Write your code here.
}
