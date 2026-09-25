// Lecture 3, slide 23 and extensions: integer ranges, bit widths, and overflow.
// Run target: lec3_integer_types
// The default run is safe and needs no input. The optional argument tests overflow.

fn main() {
    // Signed integers represent negative values; unsigned integers do not.
    let signed: i8 = -3;
    let unsigned: u8 = 42;
    println!("signed = {signed}, unsigned = {unsigned}");
    println!("i8 range: {}..={}", i8::MIN, i8::MAX);
    println!("u8 range: {}..={}", u8::MIN, u8::MAX);
    println!("usize bits for this build target: {}", usize::BITS);
    // usize/isize depend on the compilation target, not the current value.

    // Boundary literals: u8 spans 0..=255; i8 spans -128..=127.
    let num: u8 = 255;
    let minimum: i8 = -128;
    println!("valid boundaries: num = {num}, minimum = {minimum}");
    // let too_large: u8 = 256;
    // let signed_too_large: i8 = 128;
    // let negative_unsigned: u8 = -1;
    // These are rejected: the requested values cannot be represented by the type.

    // Different literal notations can represent exactly the same integer.
    let decimal = 255_u16;
    let hexadecimal = 0xff_u16;
    let binary = 0b1111_1111_u16;
    println!("equivalent literals: {decimal}, {hexadecimal}, {binary}");

    // Explicit methods make overflow behavior independent of the build profile.
    let checked = num.checked_add(1);
    let wrapped = num.wrapping_add(1);
    let saturated = num.saturating_add(1);
    let overflowed = num.overflowing_add(1);
    println!("{num}.checked_add(1) = {checked:?}"); // None: report failure.
    println!("{num}.wrapping_add(1) = {wrapped}"); // 0: wrap modulo 256.
    println!("{num}.saturating_add(1) = {saturated}"); // 255: clamp at the maximum.
    println!("{num}.overflowing_add(1) = {overflowed:?}"); // (0, true): value + flag.
    println!("254.checked_add(1) = {:?}", 254_u8.checked_add(1)); // Some(255).
    // Some(value) and None are the success/failure forms of Option.
    // The {:?} formatter displays these values and the tuple for inspection.

    // Integer division has additional failure cases, even with overflow checks off.
    println!("1 / 0, checked = {:?}", 1_u8.checked_div(0)); // None.
    println!("i8::MIN / -1, checked = {:?}", i8::MIN.checked_div(-1)); // None.
    // At runtime, ordinary integer division by zero panics in both profiles.
    // i8::MIN / -1 would produce 128, which does not fit in i8; it also panics.

    // Optional runtime experiment, using input unavailable at compilation time:
    // cargo run --bin lec3_integer_types -- 254
    // cargo run --bin lec3_integer_types -- 255
    // cargo run --release --bin lec3_integer_types -- 255
    // With default profiles: 254 gives 255 in both; 255 panics in dev, wraps in release.
    // The overflow-checks setting can change this; the profile name alone is not a rule.
    if let Some(argument) = std::env::args().nth(1) {
        let runtime_value: u8 = argument.parse().expect("Provide an integer from 0 to 255");
        println!("Runtime experiment: adding 1 to {runtime_value}");
        let next = runtime_value + 1;
        println!("Result: {next}");
    }
}
