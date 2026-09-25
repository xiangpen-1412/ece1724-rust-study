// Lecture 3, slides 24-25: floating-point types and numeric operations.
// Run target: lec3_numeric_operations
// Operand types determine arithmetic behavior before any later conversion happens.

fn main() {
    // Specify f32 explicitly, or let an unconstrained float default to f64.
    let small_float: f32 = 1.5;
    let ordinary_float = 2.5;
    println!("f32 value: {small_float}");
    println!("default float value: {ordinary_float}");

    let sum = 2 + 3;
    let product = 4 * 6;
    let quotient = 9.0 / 4.0;
    println!("sum = {sum}, product = {product}, quotient = {quotient}");

    // Integer division truncates toward zero; it does not round down.
    let integer_quotient = 7 / 3;
    let negative_quotient = -7 / 3;
    let float_quotient = 7.0 / 3.0;
    println!("7 / 3 = {integer_quotient}"); // 2, with type i32.
    println!("-7 / 3 = {negative_quotient}"); // -2, with type i32.
    println!("7.0 / 3.0 = {float_quotient}"); // Approximately 2.3333333333333335, f64.

    // A nonzero integer remainder has the same sign as the dividend.
    println!("-7 % 3 = {}", -7 % 3); // -1.
    println!("7 % -3 = {}", 7 % -3); // 1.
    println!("-7 % -3 = {}", -7 % -3); // -1.
    let dividend = -7;
    let divisor = 3;
    let remainder = dividend % divisor;
    let reconstructed = (dividend / divisor) * divisor + remainder;
    println!("reconstructed dividend = {reconstructed}"); // (-2) * 3 + (-1) = -7.

    // Converting the final result cannot recover a fraction already discarded.
    let converted_after_division = (7 / 3) as f64;
    let converted_before_division = 7_f64 / 3.0;
    println!("convert after integer division: {converted_after_division}"); // 2.0.
    println!("use floating-point operands first: {converted_before_division}");

    // Binary floating-point cannot represent many decimal fractions exactly.
    let computed = 0.1_f64 + 0.2;
    let expected = 0.3_f64;
    let absolute_error = (computed - expected).abs();
    println!("0.1 + 0.2 = {computed:.17}");
    println!("0.3       = {expected:.17}");
    println!("exact equality: {}", computed == expected); // false.
    println!("absolute error: {absolute_error:e}");

    // For these small values, 1e-12 is a deliberately chosen absolute tolerance.
    let tolerance = 1e-12;
    println!("equal within tolerance: {}", absolute_error <= tolerance); // true.
    println!("f64::EPSILON = {:e}", f64::EPSILON);
    // EPSILON is the gap from 1.0 to the next representable f64, not a universal tolerance.
    // Choose tolerances for the problem's scale and accuracy needs; relative error may matter.

    // Numeric types are not implicitly converted to make arithmetic operands match.
    let integer: i32 = 3;
    let precise: f64 = 0.5;
    // let invalid_sum = integer + precise;
    // Error: i32 cannot be added directly to f64.
    let mixed_sum = f64::from(integer) + precise;
    println!("converted i32 + f64 = {mixed_sum}"); // 3.5.

    // let invalid_float_sum = small_float + precise;
    // Error: f32 and f64 are different types, even though both store floating-point values.
    let float_sum = f64::from(small_float) + precise;
    println!("converted f32 + f64 = {float_sum}"); // 2.0.
}
