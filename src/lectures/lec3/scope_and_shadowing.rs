// Lecture 3, slide 8: explore scope, shadowing, and mutability together.
// Run target: lec3_scope_shadowing

fn main() {
    // 1. Shadowing in the same scope: the second let creates a new binding with the same name.
    // The right-hand side uses the old score of 10; the result initializes the new score.
    let score = 10;
    let score = score + 5;
    println!("Outer score after shadowing: {score}"); // 15

    // 2. Inner scope: the new binding shadows the outer name only within these braces.
    {
        let score = score * 2;
        let extra = 2;
        println!("Inner score: {score}"); // 30
        println!("Inner score plus extra: {}", score + extra); // 32
    }

    println!("Outer score after the block: {score}"); // 15, not 10 or 30.
    // println!("{extra}"); // Error: extra is out of scope here.

    // 3. Contrast: without another let, assignment updates the outer mutable variable.
    // Leaving the block does not undo this assignment.
    let mut count = 1;
    {
        count += 1;
    }
    println!("Outer count after assignment: {count}"); // 2

    // 4. A new binding does not inherit the old binding's mutability.
    let mut total = 10;
    total += 5;
    let total = total * 2;
    println!("Total after shadowing: {total}"); // 30
    // total += 1; // Error: the latest total declaration has no mut.

    // 5. Shadowing allows the new binding to have a different type.
    // See input_parsing.rs for a complete input example using .parse() and .expect().
    let value = "42";
    println!("Text value: {value}");
    let value: u32 = value.parse().expect("Expected an integer");
    println!("Numeric value plus one: {}", value + 1); // 43

    // Contrast: mut allows changing a variable's value, but not its type.
    // let mut text = "42";
    // text = 42; // Error: a string reference and an integer have different types.
}
