// Lecture 7 - Vectors and Hash Maps: Different enum variants in one vector
// Run: cargo run --bin collections_enum_elements
// TODO: Store the supplied enum values in one vector. Use match to handle each variant.

#[derive(Debug)]
#[allow(dead_code)]
enum Cell {
    Integer(i32),
    Decimal(f64),
    Text(String),
}

fn main() {
    let cells = [
        Cell::Integer(3),
        Cell::Decimal(10.12),
        Cell::Text(String::from("blue")),
    ];
    println!("Input cells: {cells:?}");
    // Write your code here.
}
