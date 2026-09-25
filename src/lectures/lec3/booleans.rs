// Lecture 3: bool, conditions, and short-circuit evaluation.
// Run target: lec3_booleans
// Printed messages reveal whether the right operand is actually evaluated.
// Deliberately invalid or panicking examples remain commented out.

fn main() {
    let has_ticket = true;
    let door_open = false;
    let can_enter = has_ticket && door_open;
    println!("是否允许进入：{can_enter}");
    println!("门是否关闭：{}", !door_open);
    if can_enter {
        println!("可以进入");
    }

    // Your original condition sketch is preserved here.
    // let mut count = 0;
    //
    // if count == 0 {
    //
    // }

    // if requires bool; Rust does not automatically treat integers as truth values.
    // count == 0 and count != 0 both produce bool, but test opposite conditions.
    let count = 0;
    // if count { } // Error: expected bool, found an integer.
    if count == 0 {
        println!("count is zero");
    }
    println!("count is nonzero: {}", count != 0); // Expected: false.

    // && skips the right operand when the left is false.
    // Expected: no helper message for the first call, a message for the second.
    let and_false = false && right_side("false &&", true);
    let and_true = true && right_side("true &&", true);
    println!("Short-circuit AND results: {and_false}, {and_true}");

    // || skips the right operand when the left is true.
    // Expected: a helper message for the first call, none for the second.
    let or_false = false || right_side("false ||", false);
    let or_true = true || right_side("true ||", false);
    println!("Short-circuit OR results: {or_false}, {or_true}");

    // On bool values, & and | compute AND and OR but evaluate both operands.
    // All four calls below print a helper message, regardless of the left value.
    // Their final bool results match the corresponding examples above.
    let eager_and_false = false & right_side("false &", true);
    let eager_and_true = true & right_side("true &", true);
    let eager_or_false = false | right_side("false |", false);
    let eager_or_true = true | right_side("true |", false);
    println!("Eager AND results: {eager_and_false}, {eager_and_true}");
    println!("Eager OR results: {eager_or_false}, {eager_or_true}");

    // A guard must run before the expression it protects.
    // Expected: false for divisor 0, true for divisor 2. Neither call panics.
    println!("Safe check with divisor 0: {}", safe_check(0));
    println!("Safe check with divisor 2: {}", safe_check(2));

    // Replacing && with & in safe_check would evaluate the division for zero.
    // Reversing the operands would also try division before checking the divisor.
    // Both changes can panic at runtime; they are not active in this example.
}

fn right_side(label: &str, value: bool) -> bool {
    println!("Right operand evaluated for {label}");
    value
}

fn safe_check(divisor: i32) -> bool {
    // The right operand is evaluated only when the nonzero check succeeds.
    divisor != 0 && 10 / divisor > 1
    // Unsafe ordering for this calculation: 10 / divisor > 1 && divisor != 0
    // Eager evaluation would also be wrong: (divisor != 0) & (10 / divisor > 1)
}
