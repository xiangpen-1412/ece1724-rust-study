// Lecture 3, slides 24-25: floating-point types and numeric operations.
// Run target: lec3_numeric_operations
// Goal: predict results from operand types, evaluation order, and signs.

fn main() {
    // Basics: specify f32 explicitly, or let an unconstrained float default to f64.
    let small_float: f32 = 1.5;
    let ordinary_float = 2.5;
    println!("f32 value: {small_float}");
    println!("default float value: {ordinary_float}");

    let sum = 2 + 3;
    let product = 4 * 6;
    let quotient = 9.0 / 4.0;
    println!("sum = {sum}, product = {product}, quotient = {quotient}");

    // Exercise A: integer division and floating-point division.
    // TODO: evaluate 7 / 3, -7 / 3, and 7.0 / 3.0.
    // Write down each expression's type and predicted result before running it.
    // My prediction:
    // My code:
    // Actual result:
    // Explanation:

    // Exercise B: remainders with negative operands.
    // TODO: evaluate -7 % 3, 7 % -3, and -7 % -3.
    // Check each result using dividend = quotient * divisor + remainder.
    // My code:

    // Exercise C: floating-point equality and rounding.
    // TODO: evaluate 0.1_f64 + 0.2, compare it with 0.3, and print the difference.
    // Choose a tolerance, compare using absolute error, and justify your tolerance.
    // My code:

    // Exercise D: can different numeric types be added directly?
    // TODO: try adding i32 to f64, and f32 to f64. Then resolve the type differences.
    // My code:
}
