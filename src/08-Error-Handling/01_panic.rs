// Lecture 6 - Error Handling: panic! and unrecoverable errors
// Run: cargo run --bin errors_panic
// TODO: Write a panic example here. Observe which statements run before the panic.
fn divide(dividend: i32, divisor: i32) -> i32 {
    if divisor == 0 {
        panic!("division by zero");
    }

    dividend / divisor
}

fn main() {
    // Write your code here.
    let vec = vec![1, 2, 3];

    // println!("{}", vec[99]);

    divide(vec[0], 0);
}
