// Type inference.

fn main() {
    // Unconstrained defaults: i32 and f64.
    let count = 10;
    let small: u8 = 10;
    let price = 3.5;

    println!(
        "count = {count}, type = {}",
        std::any::type_name_of_val(&count)
    );
    println!(
        "small = {small}, type = {}",
        std::any::type_name_of_val(&small)
    );
    println!(
        "price = {price}, type = {}",
        std::any::type_name_of_val(&price)
    );

    // Later use constrains n to u16.
    let n = 300;
    let m: u16 = n;
    println!("n = {n}, type = {}", std::any::type_name_of_val(&n));
    println!("m = {m}, type = {}", std::any::type_name_of_val(&m));
    // u8 would fail: 300 is out of range.

    let inferred = 10;
    let destination: u8 = inferred;
    println!(
        "inferred type = {}, destination = {destination}",
        std::any::type_name_of_val(&inferred)
    );

    let fixed: i32 = 10;
    // let invalid: u8 = fixed; // No implicit i32 -> u8.
    let converted = fixed as u8;
    println!("explicit conversion: {fixed} -> {converted}");
    // Narrowing casts do not check ranges.

    // Literal suffix.
    let total = 25_u64;
    println!(
        "total = {total}, type = {}",
        std::any::type_name_of_val(&total)
    );

    // mut preserves the type.
    let mut level = 1;
    level += 1;
    println!("mutable integer level = {level}");
    // level = 2.5; // Expected integer.

    // Shadowing can change the type.
    let level = 2.5;
    println!(
        "shadowed level = {level}, type = {}",
        std::any::type_name_of_val(&level)
    );
    // level = 3.5; // New binding is immutable.
}
