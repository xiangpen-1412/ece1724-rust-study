// Functions and expressions.
// Run: lec3_functions

fn main() {
    // snake_case names; definitions may follow calls.
    let total = add_numbers(7, 5);
    println!("Sum returned by add_numbers: {total}"); // 12
    println!("An explicit return gives: {}", add_with_return(7, 5)); // 12

    // Parameter types constrain literal inference.
    show_byte(10); // Inferred u8.
    let typed_number: i32 = 10;
    // show_byte(typed_number); // Error: i32, not u8.
    show_byte(typed_number as u8); // Explicit conversion.
    // show_byte(300); // Error: out of u8 range.

    // i32 is copied; the caller is unchanged.
    let original = 41;
    let incremented = increment_copy(original);
    println!("Caller: {original}; function result: {incremented}"); // 41 and 42

    // A tail expression supplies the block value.
    let expression_value = {
        let base = 3; // Local binding.
        base * 4 // 12
    };
    let statement_value = {
        12; // Discards 12; yields ().
    };
    println!("Expression block: {expression_value}; statement block: {statement_value:?}");
    // let invalid = (let x = 3); // Error: let is not a value here.

    // Assignment evaluates to ().
    let mut destination = 0;
    println!("Before assignment: {destination}");
    let assignment_value = { destination = 9 };
    println!("After assignment: {destination}; block value: {assignment_value:?}"); // 9 and ()

    // Printing and returning differ.
    let printed_result = print_total(total); // Prints 12; returns ().
    println!("Value returned by print_total: {printed_result:?}"); // ()
    // let wrong: i32 = print_total(total); // Error: expected i32, found ().
    // println!("{}", print_total(total)); // Error: () needs {:?}.

    // TODO: Write adjust_score(score: i32, bonus: i32) -> i32 using a tail expression.
    // Test (70, 5) and (90, -10); distinguish printing from returning.
}

// Parameters need types.
fn add_numbers(left: i32, right: i32) -> i32 {
    left + right // Tail return.
}

fn add_with_return(left: i32, right: i32) -> i32 {
    return left + right; // Explicit return.
}

fn show_byte(value: u8) {
    println!("u8 parameter: {value}");
    // value = 20; // Error: immutable parameter.
}

fn increment_copy(mut value: i32) -> i32 {
    value += 1;
    value
}

// No -> means a () return type.
fn print_total(total: i32) {
    println!("Printed total: {total}");
}

// fn missing_parameter_type(value) -> i32 { value }
// Error: missing parameter type.
// fn missing_return_type() { 12 }
// Error: implicit () return type.
// fn discarded_return() -> i32 { 12; }
// Error: the semicolon discards 12.
