// Run: lec3_variables

fn main() {
    // Bindings are immutable unless marked mut.
    let immutable = 1;
    println!("Immutable value: {immutable}");

    let mut mutable = 1;
    println!("Before mutation: {mutable}");
    mutable += 1;
    println!("After mutation: {mutable}");

    let mut score = 10;
    score = score + 5;
    println!("Score: {score}");

    // immutable += 1; // Error: immutable binding.
    // Removing mut from score also prevents reassignment.
}
