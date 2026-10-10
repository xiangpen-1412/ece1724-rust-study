// ECE1724 — Lecture 5: Struct Debugging and Refactoring (Book 5.2)
// Pages: 15-19 (Lecture 5 slides; title page = 1).
// Topics: derive(Debug), debug printing, Rectangle, borrowing a struct.

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Before refactoring: the two dimensions are separate arguments.
fn area_from_dimensions(width: u32, height: u32) -> u32 {
    width * height
}

// After refactoring: borrow one rectangle without taking ownership.
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Before refactoring: {}", area_from_dimensions(30, 50)); // 1500
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    ); // 1500

    // Debug printing borrows the value; # requests pretty formatting.
    println!("Compact: {rect1:?}");
    println!("Pretty: {rect1:#?}");
    println!("Still usable: width = {}", rect1.width); // 30
    // println!("{rect1}"); // Error: Debug does not also implement Display.
    // area(rect1); // Error: the argument must be &Rectangle.

    // Expand (leave for practice):
    // TODO 1: Compare a borrowed area function with a by-value version.
    // Try printing rect1 afterward; explain which version moves it and why.
    // TODO 2: Put Rectangle inside a Panel, then derive Debug for Panel.
    // Remove Debug from the inner type and explain the compiler error.
}
