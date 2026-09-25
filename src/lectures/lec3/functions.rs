// Lecture 3, slides 36-37: functions, parameters, statements, and expressions.
// Run target: lec3_functions
// Read the complete examples first; only the final exercise is left unfinished.

fn main() {
    // 1. Function names conventionally use snake_case. Definitions may come later.
    let total = add_numbers(7, 5);
    println!("Sum returned by add_numbers: {total}"); // 12
    println!("An explicit return gives: {}", add_with_return(7, 5)); // 12

    // 2. The expected parameter type can determine an unsuffixed literal's type.
    show_byte(10); // The literal is inferred as u8, matching the parameter.
    let typed_number: i32 = 10;
    // show_byte(typed_number); // Error: expected u8, found i32; no implicit conversion.
    show_byte(typed_number as u8); // Explicit conversion; 10 fits in u8.
    // show_byte(300); // Error: this literal does not fit in the required u8 type.

    // 3. An i32 parameter receives a copy. Local mutability does not modify the caller.
    let original = 41;
    let incremented = increment_copy(original);
    println!("Caller: {original}; function result: {incremented}"); // 41 and 42

    // 4. A block's final expression supplies its value when it has no semicolon.
    let expression_value = {
        let base = 3; // A let statement creates a binding inside this block.
        base * 4 // The block evaluates to this i32 expression: 12.
    };
    let statement_value = {
        12; // The semicolon discards the value; this block evaluates to ().
    };
    println!("Expression block: {expression_value}; statement block: {statement_value:?}");
    // let invalid = (let x = 3); // Error: a let statement is not a value expression here.

    // 5. Assignment is itself an expression whose value is unit, not the assigned value.
    let mut destination = 0;
    println!("Before assignment: {destination}");
    let assignment_value = { destination = 9 };
    println!("After assignment: {destination}; block value: {assignment_value:?}"); // 9 and ()
    // Even without a final semicolon, this block produces unit: its tail is assignment.

    // 6. Printing is an observable action; returning supplies a value to the caller.
    let printed_result = print_total(total); // Prints 12, then returns ().
    println!("Value returned by print_total: {printed_result:?}"); // ()
    // let wrong: i32 = print_total(total); // Error: expected i32, found ().
    // println!("{}", print_total(total)); // Error: () has no Display formatter; use {:?}.

    // TODO 1 of 2: write adjust_score(score: i32, bonus: i32) -> i32 below main.
    // Create a local result = score + bonus, then return it with a tail expression.
    // Call the function with (70, 5) and (90, -10), and print the returned values.
    // Explain why printing result inside the function cannot replace returning result.
    // My prediction:
    // My code:
    // Actual result and explanation:
}

// Each parameter needs a type. The arrow declares the function's return type.
fn add_numbers(left: i32, right: i32) -> i32 {
    left + right // No semicolon: this is the value returned to the caller.
}

fn add_with_return(left: i32, right: i32) -> i32 {
    return left + right; // Explicit return exits the function with this value.
}

fn show_byte(value: u8) {
    println!("u8 parameter: {value}");
    // value = 20; // Error: function parameters are immutable unless declared mut.
}

fn increment_copy(mut value: i32) -> i32 {
    value += 1;
    value
}

// Omitting -> means the return type is (); it is not inferred from the function body.
fn print_total(total: i32) {
    println!("Printed total: {total}");
}

// Invalid variants are commented out so the reference remains runnable:
// fn missing_parameter_type(value) -> i32 { value }
// Error: parameter types must be written in a normal function signature.
// fn missing_return_type() { 12 }
// Error: the declared return type is implicitly (), but the body supplies i32.
// fn discarded_return() -> i32 { 12; }
// Error: the body evaluates to (), but the function promises i32.
