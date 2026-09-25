// Run: lec3_scope_shadowing

fn main() {
    // Each let creates a new binding.
    let score = 10;
    let score = score + 5;
    println!("Outer score after shadowing: {score}"); // 15

    // Inner shadowing ends with this block.
    {
        let score = score * 2;
        let extra = 2;
        println!("Inner score: {score}"); // 30
        println!("Inner score plus extra: {}", score + extra); // 32
    }

    println!("Outer score after the block: {score}"); // 15, not 10 or 30.
    // println!("{extra}"); // Error: out of scope.

    // Assignment changes the existing outer binding.
    let mut count = 1;
    {
        count += 1;
    }
    println!("Outer count after assignment: {count}"); // 2

    // Shadowing does not inherit mutability.
    let mut total = 10;
    total += 5;
    let total = total * 2;
    println!("Total after shadowing: {total}"); // 30
    // total += 1; // Error: new binding is immutable.

    // Shadowing can change type; assignment cannot.
    let value = "42";
    println!("Text value: {value}");
    let value: u32 = value.parse().expect("Expected an integer");
    println!("Numeric value plus one: {}", value + 1); // 43

    // let mut text = "42";
    // text = 42; // Error: incompatible types.
}
