// Lecture 3, slides 37-40: early return and if expressions.
// Run target: lec3_if_expressions
// Complete examples first; only the final combined exercise is left open.

fn main() {
    // Ordinary if conditions require bool; comparisons produce bool.
    let number = 5;
    if number < 0 {
        println!("negative");
    } else if number > 0 {
        println!("positive");
    } else {
        println!("zero");
    }
    // if number { } // Error: an integer is not a bool.
    // if (number = 0) { } // Error: assignment produces (), not bool.

    // An else-if chain selects only the first matching branch.
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

    // if can produce a value. Inner tail expressions supply that value;
    // the semicolon after the closing brace ends the surrounding let statement.
    let enabled = true;
    let amount: i32 = if enabled { 10 } else { 20 };
    println!("Selected amount: {amount}"); // 10
    let unit = if enabled {
        println!("Only this branch executes");
    } else {
        println!("This branch is skipped");
    };
    println!("Both printing branches produce unit: {unit:?}"); // ()

    // Unselected branches are still type-checked.
    // let mismatch = if true { 10 } else { "twenty" }; // Error: integer versus &str.
    // let mismatch = if enabled { 10 } else { 20; }; // Error: integer versus ().
    // let missing_else = if enabled { 10 }; // Error: no-else if must produce unit.
    // let fixed: f64 = if enabled { 10 } else { 20.0 }; // Error: no integer-to-float coercion.
    let float_amount: f64 = if enabled { 10.0 } else { 20.0 };
    println!("Matching float branches: {float_amount:.1}");

    // A chosen branch may execute statements before producing its final value.
    println!("Scaled 9: {}", scale_small(9)); // 90
    println!("Scaled 10: {}", scale_small(10)); // 5
    println!("Scaled -9: {}", scale_small(-9)); // -90
    println!("Scaled -10: {}", scale_small(-10)); // -5
    println!("Scaled -11: {}", scale_small(-11)); // -5: division truncates toward zero.

    // Guard clauses return before a dangerous operation can run.
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

    // TODO 2 of 2: implement clamp_score(score: i32) -> (i32, bool).
    // Return the score limited to 0..=100, and a flag indicating whether it changed.
    // Use if/else to select the score, then return one tuple using a tail expression.
    // Call it with -5, 70, and 120. Print the results and explain all return types.
    // My prediction:
    // My code:
    // Actual result and explanation:
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
    // Advanced: return exits this entire function, not just the if block.
    // This branch supplies no value to remainder; the continuing branch supplies u32.
    let remainder = if divisor == 0 {
        return false;
    } else {
        dividend % divisor
    };
    remainder == 0
}
