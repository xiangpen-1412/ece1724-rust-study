// ECE1724 — Lecture 5: The Option Enum (Book 6.1)
// Pages: 37-46 (Lecture 5 slides; title page = 1).
// Topics: Option<T>, Some(T), None, type annotations, absent values.

// The standard library already defines this shape; do not redefine it:
// enum Option<T> { None, Some(T) }
// Option, Some, and None are available without a use statement.

fn main() {
    // Different T means different Option types.
    let some_number = Some(5); // Option<i32>
    let some_char = Some('e'); // Option<char>
    println!("number = {some_number:?}, char = {some_char:?}");

    // None contains no value from which to infer T.
    let mut absent_number: Option<i32> = None;
    println!("absent number = {absent_number:?}");
    // let unknown = None; // Error without other context that determines T.
    // let mixed = [some_number, some_char]; // Error: different Option types.

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // let sum = x + y; // Error: i8 and Option<i8> are different types.

    // Extract the value and handle absence explicitly; match is next in class.
    let sum = match y {
        Some(value) => Some(x + value),
        None => None,
    };
    println!("sum = {sum:?}"); // Some(10)

    match absent_number {
        Some(value) => println!("Present number: {value}"),
        None => println!("No number available"),
    }

    absent_number = Some(6);
    let num = absent_number.unwrap_or(0);
    println!("{}", num);
    // None is an ordinary enum variant, not a null reference to dereference.
}

// Expand (leave for practice):
// TODO: Write safe_divide(a, b) -> Option<i32>; (10, 2) -> Some(5), (10, 0) -> None.
// TODO: Distinguish Some(0) from None with match; print both cases explicitly.
