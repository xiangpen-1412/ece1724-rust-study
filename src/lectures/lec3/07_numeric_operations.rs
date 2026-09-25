// Numeric operations.

fn main() {
    let small_float: f32 = 1.5;
    let ordinary_float = 2.5;
    println!("f32 value: {small_float}");
    println!("default float value: {ordinary_float}");

    let sum = 2 + 3;
    let product = 4 * 6;
    let quotient = 9.0 / 4.0;
    println!("sum = {sum}, product = {product}, quotient = {quotient}");

    // Integer division truncates toward zero.
    let integer_quotient = 7 / 3;
    let negative_quotient = -7 / 3;
    let float_quotient = 7.0 / 3.0;
    println!("7 / 3 = {integer_quotient}");
    println!("-7 / 3 = {negative_quotient}");
    println!("7.0 / 3.0 = {float_quotient}");

    // Nonzero remainder follows the dividend sign.
    println!("-7 % 3 = {}", -7 % 3);
    println!("7 % -3 = {}", 7 % -3);
    println!("-7 % -3 = {}", -7 % -3);
    let dividend = -7;
    let divisor = 3;
    let remainder = dividend % divisor;
    let reconstructed = (dividend / divisor) * divisor + remainder;
    println!("reconstructed dividend = {reconstructed}");

    // Cast timing matters.
    let converted_after_division = (7 / 3) as f64;
    let converted_before_division = 7_f64 / 3.0;
    println!("convert after integer division: {converted_after_division}");
    println!("use floating-point operands first: {converted_before_division}");

    // Floating-point rounding.
    let computed = 0.1_f64 + 0.2;
    let expected = 0.3_f64;
    let absolute_error = (computed - expected).abs();
    println!("0.1 + 0.2 = {computed:.17}");
    println!("0.3       = {expected:.17}");
    println!("exact equality: {}", computed == expected); // false
    println!("absolute error: {absolute_error:e}");

    // Choose tolerance for the problem scale.
    let tolerance = 1e-12;
    println!("equal within tolerance: {}", absolute_error <= tolerance);
    println!("f64::EPSILON = {:e}", f64::EPSILON);
    // EPSILON: gap above 1.0, not a universal tolerance.

    let integer: i32 = 3;
    let precise: f64 = 0.5;
    // let invalid_sum = integer + precise; // i32 + f64 is invalid.
    let mixed_sum = f64::from(integer) + precise;
    println!("converted i32 + f64 = {mixed_sum}");

    // let invalid_float_sum = small_float + precise; // f32 + f64 is invalid.
    let float_sum = f64::from(small_float) + precise;
    println!("converted f32 + f64 = {float_sum}");
}
