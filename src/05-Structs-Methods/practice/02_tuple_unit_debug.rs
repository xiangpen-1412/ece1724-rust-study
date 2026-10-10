/*
02 - Tuple structs, unit structs, and Debug (Book 5.1-5.2)

Question 1 - Same shape, different types

Define Color and Point as tuple structs with three u8 fields each.
Define a unit struct Marker. Do not derive Copy or Clone.
Predict compilation and output for each independent snippet, then verify.

    let p: Point = Color(1, 2, 3);                // A

    let c = Color(1, 2, 3);
    let Color(r, g, b) = c;
    println!("{} {}", r + g + b, c.0);           // B

    let m = Marker;
    let another = m;
    let last = m;                               // C

    let empty: () = Marker;                     // D

Explicitly convert a Color into a Point without type aliases, as, or unsafe.
Can 'all fields are Copy' alone explain the difference between B and C?

Question 2 - Debug and ownership of function arguments

Define Rectangle { width: u32, height: u32 } and
Panel { title: String, rect: Rectangle }.
Write a free function area so main can calculate the same Rectangle's area
twice. Then print the same Panel with {:?} and {:#?}, keeping panel.title
usable afterward. Choose the parameter type and when to put Rectangle in Panel.

Introduce each fault independently:
1. Derive Debug for Panel but not Rectangle.
2. Make area take Rectangle by value and keep a later use of the original.

Record each compiler explanation and make a minimal repair without clone,
Copy, or Clone derives. After fixing Debug, would {} work instead of {:?}?
*/

// TODO: Define the required types and functions yourself.

fn main() {
    // Question 1: Record predictions, then write and verify your code.
    // TODO: Add your work here.

    // Question 2: Record predictions and explain your implementation.
    // TODO: Add your work here.
}
