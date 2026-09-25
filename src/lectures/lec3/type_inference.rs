// Lecture 3, slides 10-21: static typing and type inference.
// Run target: lec3_type_inference
// Invalid examples stay commented so the complete reference remains runnable.

fn main() {
    // Unconstrained integer and float literals default to i32 and f64.
    let count = 10;
    let small: u8 = 10;
    let price = 3.5;

    // Observe the actual inferred types; inference happens during compilation.
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

    // Later uses can constrain an earlier binding's type.
    let n = 300;
    let m: u16 = n;
    println!("n = {n}, type = {}", std::any::type_name_of_val(&n));
    println!("m = {m}, type = {}", std::any::type_name_of_val(&m));
    // Both are u16. There is no temporary i32 value converted into a u16.
    // Replacing u16 with u8 above fails: the literal 300 cannot fit in u8.

    // Inference can select u8 when no incompatible type was specified.
    let inferred = 10;
    let destination: u8 = inferred;
    println!(
        "inferred type = {}, destination = {destination}",
        std::any::type_name_of_val(&inferred)
    );

    // An explicit type annotation removes that freedom.
    let fixed: i32 = 10;
    // let invalid: u8 = fixed;
    // Error: expected u8, found i32. Fitting in the range is not enough.
    let converted = fixed as u8;
    println!("explicit conversion: {fixed} -> {converted}");
    // This cast preserves 10, but narrowing casts do not generally check ranges.

    // A literal suffix is another way to specify a type.
    let total = 25_u64;
    println!(
        "total = {total}, type = {}",
        std::any::type_name_of_val(&total)
    );

    // mut permits changing a value; it does not permit changing its type.
    let mut level = 1;
    level += 1;
    println!("mutable integer level = {level}");
    // level = 2.5;
    // Error: an integer binding cannot be assigned a floating-point value.

    // Shadowing creates a new binding, which may have a different type.
    let level = 2.5;
    println!(
        "shadowed level = {level}, type = {}",
        std::any::type_name_of_val(&level)
    );
    // level = 3.5;
    // Error: this new binding is immutable; the earlier mut is not inherited.
}
