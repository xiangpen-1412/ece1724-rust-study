// Lecture 3: bool, conditions, and short-circuit evaluation.
// Run target: lec3_booleans
// Start with && and ! in the basic example, then implement the exercises yourself.
// Distinguish compilation failures, a normal false result, and a runtime panic.

fn main() {
    let has_ticket = true;
    let door_open = false;
    let can_enter = has_ticket && door_open;
    println!("是否允许进入：{can_enter}");
    println!("门是否关闭：{}", !door_open);
    if can_enter {
        println!("可以进入");
    }

    // TODO 1: Investigate the type required by an if condition.
    // Declare an integer count and try using it directly as the condition.
    // Record the compiler feedback, then change the condition to count != 0.
    // Comment out the deliberately invalid version and keep the working version.
    // My prediction:
    // Actual result:
    // Explanation:
    // Write your code here:

    // TODO 2: Observe short-circuiting through output, not just the final bool.
    // Write a small function that prints "right side evaluated" and returns bool.
    // Call it on the right of false &&, true &&, false ||, and true ||.
    // Which cases print the message? Record all four cases.
    // My prediction:
    // Actual result:
    // Explanation:
    // Write the calls here; the helper function can be defined outside main:

    // TODO 3: Compare && with &, and || with |.
    // Reuse the printing function and change only the operator. Observe the right side.
    // Explain why short-circuiting matters when checking a divisor before division.
    // My prediction:
    // Actual result:
    // Explanation:
    // Write your code here:
}
