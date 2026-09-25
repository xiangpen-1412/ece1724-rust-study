// Integer ranges and overflow.

fn main() {
    let signed: i8 = -3;
    let unsigned: u8 = 42;
    println!("signed = {signed}, unsigned = {unsigned}");
    println!("i8 range: {}..={}", i8::MIN, i8::MAX);
    println!("u8 range: {}..={}", u8::MIN, u8::MAX);
    println!("usize bits for this build target: {}", usize::BITS);
    // usize/isize width depends on the target.

    let num: u8 = 255;
    let minimum: i8 = -128;
    println!("valid boundaries: num = {num}, minimum = {minimum}");
    // let too_large: u8 = 256; // Out of range.
    // let signed_too_large: i8 = 128; // Out of range.
    // let negative_unsigned: u8 = -1; // Unsigned.

    // Literal bases.
    let decimal = 255_u16;
    let hexadecimal = 0xff_u16;
    let binary = 0b1111_1111_u16;
    println!("equivalent literals: {decimal}, {hexadecimal}, {binary}");

    // Explicit overflow behavior, independent of profile.
    let checked = num.checked_add(1);
    let wrapped = num.wrapping_add(1);
    let saturated = num.saturating_add(1);
    let overflowed = num.overflowing_add(1);
    println!("{num}.checked_add(1) = {checked:?}"); // None
    println!("{num}.wrapping_add(1) = {wrapped}"); // 0
    println!("{num}.saturating_add(1) = {saturated}"); // 255
    println!("{num}.overflowing_add(1) = {overflowed:?}"); // (0, true)
    println!("254.checked_add(1) = {:?}", 254_u8.checked_add(1));

    // Ordinary / panics for zero or MIN / -1, even in release.
    println!("1 / 0, checked = {:?}", 1_u8.checked_div(0));
    println!("i8::MIN / -1, checked = {:?}", i8::MIN.checked_div(-1));

    // cargo run --bin lec3_integer_types -- 254
    // cargo run --bin lec3_integer_types -- 255
    // cargo run --release --bin lec3_integer_types -- 255
    // Defaults: 255 + 1 panics in dev, wraps in release; overflow-checks can override.
    if let Some(argument) = std::env::args().nth(1) {
        let runtime_value: u8 = argument.parse().expect("Provide an integer from 0 to 255");
        println!("Runtime experiment: adding 1 to {runtime_value}");
        let next = runtime_value + 1;
        println!("Result: {next}");
    }
}
