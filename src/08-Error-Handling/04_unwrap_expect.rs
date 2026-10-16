// Lecture 6 - Error Handling: unwrap and expect
// Run: cargo run --bin errors_unwrap_expect
// TODO: Try unwrap and expect with the sample file. Then compare failure messages using the missing path.

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let sample_path = "src/08-Error-Handling/data/hello.txt";
    let missing_path = "src/08-Error-Handling/data/missing.txt";
    println!("Sample path: {sample_path}");
    println!("Missing path: {missing_path}");

    File::open(missing_path).unwrap_or_else(|e| {
        if e.kind() == ErrorKind::NotFound {
            File::create(missing_path).unwrap_or_else(|e| {
                panic!("Tried to create file but there was a problem: {:?}", e)
            })
        } else {
            panic!("Failed to open \"{}\". Error: {}", missing_path, e);
        }
    });

    // Write your code here.
}
