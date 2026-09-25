// if expressions and early return.
// Run: lec3_if_expressions

fn main() {
    let number = 5;
    if number < 0 {
        println!("negative");
    } else if number > 0 {
        println!("positive");
    } else {
        println!("zero");
    }
    // if number { } // Error: not bool.
    // if (number = 0) { } // Error: assignment yields ().

    // First matching branch wins.
    let score = 95;
    let misleading_grade = if score >= 60 {
        'C'
    } else if score >= 90 {
        'A'
    } else {
        'F'
    };
    let grade = if score >= 90 {
        'A'
    } else if score >= 60 {
        'C'
    } else {
        'F'
    };
    println!("Broad condition first: {misleading_grade}; specific first: {grade}"); // C, A

    // Branch tails supply values; the outer semicolon ends let.
    let enabled = true;
    let amount: i32 = if enabled { 10 } else { 20 };
    println!("Selected amount: {amount}"); // 10
    let unit = if enabled {
        println!("Only this branch executes");
    } else {
        println!("This branch is skipped");
    };
    println!("Both printing branches produce unit: {unit:?}"); // ()

    // Every branch is type-checked.
    // let mismatch = if true { 10 } else { "twenty" }; // Error: integer versus &str.
    // let mismatch = if enabled { 10 } else { 20; }; // Error: integer versus ().
    // let missing_else = if enabled { 10 }; // Error: missing else for a value.
    // let fixed: f64 = if enabled { 10 } else { 20.0 }; // Error: integer/float mismatch.
    let float_amount: f64 = if enabled { 10.0 } else { 20.0 };
    println!("Matching float branches: {float_amount:.1}");

    println!("Scaled 9: {}", scale_small(9)); // 90
    println!("Scaled 10: {}", scale_small(10)); // 5
    println!("Scaled -9: {}", scale_small(-9)); // -90
    println!("Scaled -10: {}", scale_small(-10)); // -5
    println!("Scaled -11: {}", scale_small(-11)); // -5: truncates toward zero.

    // Return before invalid division.
    println!("12 divisible by 3: {}", is_divisible_by(12, 3)); // true
    println!("12 divisible by 5: {}", is_divisible_by(12, 5)); // false
    println!("12 divisible by 0: {}", is_divisible_by(12, 0)); // false, no panic
    println!("0 divisible by 3: {}", is_divisible_by(0, 3)); // true
    println!(
        "Expression form, divisor 0: {}",
        is_divisible_expression(12, 0)
    ); // false
    println!(
        "Expression form, divisor 3: {}",
        is_divisible_expression(12, 3)
    ); // true

    // TODO: clamp_score(i32) -> (i32, bool): clamp to 0..=100 and flag changes.
    // Use if/else and a tail tuple; test -5, 70, and 120.
}

fn scale_small(number: i32) -> i32 {
    if number > -10 && number < 10 {
        println!("Scale branch: multiply by ten");
        number * 10
    } else {
        println!("Scale branch: divide by two");
        number / 2
    }
}

fn is_divisible_by(dividend: u32, divisor: u32) -> bool {
    if divisor == 0 {
        return false;
    }
    dividend % divisor == 0
}

fn is_divisible_expression(dividend: u32, divisor: u32) -> bool {
    // return exits the function; only the continuing branch supplies remainder.
    let remainder = if divisor == 0 {
        return false;
    } else {
        dividend % divisor
    };
    remainder == 0
}
