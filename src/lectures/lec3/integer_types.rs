// Lecture 3, slide 23 and extensions: integer ranges, bit widths, and overflow.
// Run target: lec3_integer_types
// Goal: distinguish compile errors, runtime overflow, and explicit overflow handling.

fn main() {
    // Basics: signed and unsigned integers.
    let signed: i8 = -3;
    let unsigned: u8 = 42;
    println!("signed = {signed}, unsigned = {unsigned}");
    println!("i8 range: {}..={}", i8::MIN, i8::MAX);
    println!("u8 range: {}..={}", u8::MIN, u8::MAX);
    println!("usize bits for this build target: {}", usize::BITS);

    // Exercise A: does a literal fit within the specified type?
    // TODO: declare u8 variables with 255 and 256, and i8 variables with -128 and 128.
    // Try one case at a time. Record compile errors, then comment out the failing code.
    // My prediction:
    // My code:
    // Actual result:
    // Explanation:

    // Exercise B: addition overflow at runtime.
    // TODO: follow input_parsing.rs to read a u8 at runtime, then add 1 to it.
    // Try inputs 254 and 255, comparing development and release builds.
    // Record the build profile. Use runtime input rather than a statically known overflow.
    // My code:

    // Exercise C: implement four explicit ways to handle overflow.
    // TODO: call checked_add, wrapping_add, saturating_add, and overflowing_add
    // on the same u8 value. Predict and print the results, then explain each use case.
    // My code:

    // Tricky cases: integer division by zero, and a signed minimum divided by -1.
    // TODO: predict whether disabling ordinary addition overflow checks makes these
    // cases behave the same way as overflowing addition. Verify with runtime inputs.
    // My code:
}
