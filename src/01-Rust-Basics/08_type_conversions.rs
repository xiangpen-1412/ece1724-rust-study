// Numeric conversions.

fn main() {
    // Widening.
    let small: u8 = 42;
    let wider: u16 = small as u16;
    let decimal: f64 = small as f64;
    println!("原值：{small}");
    println!("转成 u16：{wider}");
    println!("转成 f64：{decimal:.1}");

    // Narrowing keeps low bits: 300 -> 44.
    let large: u16 = 300;
    let narrowed = large as u8;
    println!("300_u16 as u8 = {narrowed}");

    // let direct: u8 = large; // No implicit u16 -> u8.
    // let literal: u8 = 300; // Out of range.
    let explicit: u8 = large as u8;
    println!("Explicit narrowing: {explicit}");

    // Division before/after conversion: 2.0 versus 2.5.
    let convert_after = (5 / 2) as f64;
    let convert_before = 5 as f64 / 2.0;
    println!("Divide first, then convert: {convert_after:.1}");
    println!("Convert first, then divide: {convert_before:.1}");

    // From: infallible conversion.
    let via_from = u16::from(small);
    println!("u16::from(42_u8) = {via_from}");

    // TryFrom: Ok or Err.
    let fits = u8::try_from(200_u16);
    let too_large = u8::try_from(large);
    println!("Checked conversion of 200: {fits:?}");
    match too_large {
        Ok(value) => println!("Checked conversion of 300: {value}"),
        Err(_) => println!("Checked conversion of 300: out of range"),
    }

    // Float -> integer: truncate toward zero.
    let positive = 3.9_f64 as i32;
    let negative = (-3.9_f64) as i32;
    println!("Float to integer: {positive}, {negative}");

    // Float casts clamp; NaN -> 0. Expected: 255, 0, 0.
    println!(
        "Float cast boundaries: {}, {}, {}",
        300.0_f64 as u8,
        (-1.0_f64) as u8,
        f64::NAN as u8
    );

    // f32 loses integer precision here: 16777217 -> 16777216.
    let exact_integer: u32 = 16_777_217;
    let approximate = exact_integer as f32;
    println!("Integer {exact_integer} converted to f32: {approximate:.0}");
}
