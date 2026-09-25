// Lecture 3: Numeric type conversions.
// Run target: lec3_type_conversions
// Run the basic example, then complete the exercises one change at a time.
// Distinguish type inference, explicit conversion, and the order of operations.

fn main() {
    let small: u8 = 42;
    let wider: u16 = small as u16;
    let decimal: f64 = small as f64;
    println!("原值：{small}");
    println!("转成 u16：{wider}");
    println!("转成 f64：{decimal:.1}");

    // TODO 1: Convert a wider integer to a narrower integer.
    // Create a u16 with the value 300, cast it to u8 with as, and print it.
    // Predict the result before running. Explain why the original value is not enough.
    // My prediction:
    // Actual result:
    // Explanation:
    // Write your code here:

    // TODO 2: Decide when the conversion happens.
    // Compare (5 / 2) as f64 with 5 as f64 / 2.0.
    // Describe the order of operations in each expression, then run both.
    // My prediction:
    // Actual result:
    // Explanation:
    // Write your code here:

    // TODO 3: Explain compilation failures as well as successful results.
    // Try assigning a u16 variable directly to a u8 variable, then try an as cast.
    // Which version compiles? Does as guarantee that the numeric value is preserved?
    // Keep one experiment active at a time. Record and comment out deliberate errors.
    // My prediction:
    // Actual result:
    // Explanation:
    // Write your code here:
}
