// Lecture 7 - Vectors and Hash Maps: Adding and removing elements
// Run: cargo run --bin collections_vector_updates
// TODO: Create a mutable vector. Try push and pop, including pop on an empty vector.

fn main() {
    // Write your code here.
    let mut v = vec![1, 2, 3, 4, 5];

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50; // dereference the mutable reference
    }

}
