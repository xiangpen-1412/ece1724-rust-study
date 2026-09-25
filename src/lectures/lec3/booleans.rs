// Boolean evaluation.
// Run: lec3_booleans

fn main() {
    let has_ticket = true;
    let door_open = false;
    let can_enter = has_ticket && door_open;
    println!("是否允许进入：{can_enter}");
    println!("门是否关闭：{}", !door_open);
    if can_enter {
        println!("可以进入");
    }

    // let mut count = 0;
    // if count == 0 {
    // }

    // Conditions require bool.
    let count = 0;
    // if count { } // Error: not bool.
    if count == 0 {
        println!("count is zero");
    }
    println!("count is nonzero: {}", count != 0); // false.

    // && skips right when left is false.
    let and_false = false && right_side("false &&", true);
    let and_true = true && right_side("true &&", true);
    println!("Short-circuit AND results: {and_false}, {and_true}");

    // || skips right when left is true.
    let or_false = false || right_side("false ||", false);
    let or_true = true || right_side("true ||", false);
    println!("Short-circuit OR results: {or_false}, {or_true}");

    // & and | always evaluate both operands.
    let eager_and_false = false & right_side("false &", true);
    let eager_and_true = true & right_side("true &", true);
    let eager_or_false = false | right_side("false |", false);
    let eager_or_true = true | right_side("true |", false);
    println!("Eager AND results: {eager_and_false}, {eager_and_true}");
    println!("Eager OR results: {eager_or_false}, {eager_or_true}");

    // Check the divisor before dividing.
    println!("Safe check with divisor 0: {}", safe_check(0));
    println!("Safe check with divisor 2: {}", safe_check(2));

}

fn right_side(label: &str, value: bool) -> bool {
    println!("Right operand evaluated for {label}");
    value
}

fn safe_check(divisor: i32) -> bool {
    divisor != 0 && 10 / divisor > 1
    // Panics for zero: 10 / divisor > 1 && divisor != 0
    // Also panics: (divisor != 0) & (10 / divisor > 1)
}
