// Lecture 3: Numeric type conversions.
// Run target: lec3_type_conversions
// These reference examples distinguish inference, conversion, and operation order.
// Deliberately invalid examples stay commented out so the whole file can run.

fn main() {
    // Basic widening: every u8 value can be represented exactly by u16 or f64.
    let small: u8 = 42;
    let wider: u16 = small as u16;
    let decimal: f64 = small as f64;
    println!("原值：{small}");
    println!("转成 u16：{wider}");
    println!("转成 f64：{decimal:.1}");

    // Narrowing keeps the low 8 bits here; it does not check the numeric range.
    // Expected: 300 becomes 44, which is 300 modulo 256.
    let large: u16 = 300;
    let narrowed = large as u8;
    println!("300_u16 as u8 = {narrowed}");

    // Direct assignment does not implicitly convert between integer types.
    // let direct: u8 = large; // Error: expected u8, found u16.
    // let literal: u8 = 300;  // Error by default: the literal is out of range.
    // An explicit narrowing cast compiles even when it changes the numeric value.
    let explicit: u8 = large as u8;
    println!("Explicit narrowing: {explicit}");

    // Convert after integer division: the fractional part has already been lost.
    // Expected: 2.0, followed by 2.5.
    let convert_after = (5 / 2) as f64;
    let convert_before = 5 as f64 / 2.0;
    println!("Divide first, then convert: {convert_after:.1}");
    println!("Convert first, then divide: {convert_before:.1}");

    // from expresses an available infallible conversion without an as cast.
    // u16::from(u8) is valid because every u8 value fits in u16.
    let via_from = u16::from(small);
    println!("u16::from(42_u8) = {via_from}");

    // try_from reports whether the value fits instead of silently narrowing it.
    // Result has two alternatives: Ok(converted_value) and Err(error).
    // Expected: the first conversion succeeds and the second is rejected.
    let fits = u8::try_from(200_u16);
    let too_large = u8::try_from(large);
    println!("Checked conversion of 200: {fits:?}");
    match too_large {
        Ok(value) => println!("Checked conversion of 300: {value}"),
        Err(_) => println!("Checked conversion of 300: out of range"),
    }

    // Float-to-integer casts discard the fractional part toward zero.
    // Expected: 3 and -3. This differs from rounding to the nearest integer.
    let positive = 3.9_f64 as i32;
    let negative = (-3.9_f64) as i32;
    println!("Float to integer: {positive}, {negative}");

    // These casts clamp values outside the integer range; NaN converts to zero.
    // Expected: 255, 0, 0. Integer narrowing and float casts use different rules.
    println!(
        "Float cast boundaries: {}, {}, {}",
        300.0_f64 as u8,
        (-1.0_f64) as u8,
        f64::NAN as u8
    );

    // Converting an integer to a float can lose precision even without overflow.
    // f32 cannot represent every integer above 2^24 exactly.
    // Expected: 16777217 becomes 16777216 when represented by this f32.
    let exact_integer: u32 = 16_777_217;
    let approximate = exact_integer as f32;
    println!("Integer {exact_integer} converted to f32: {approximate:.0}");
}
