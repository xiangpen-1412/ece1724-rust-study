# 09 Booleans and Short Circuiting

[[00 Lecture 03 Overview|Lecture 3 overview]] · [[90 Code Map|Code map]]

Source: `src/lectures/lec3/09_booleans.rs` ([GitHub](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/09_booleans.rs)).

Run: `cargo run --bin lec3_booleans`

**Conditions require `bool`, whose values are `true` and `false`.** Rust does not automatically treat an integer as a truth value. With an integer `count`, `if count` is invalid; `if count != 0` uses an explicit comparison that produces a boolean.

`count == 0` is also valid, but asks the opposite question. Assignment with `=` is different from equality with `==`; assignment does not supply the boolean result needed by an `if` condition. `!` negates a boolean.

**Short circuiting determines whether the right operand runs.**

| Expression | Evaluate the right operand? |
| --- | --- |
| `false && expression` | No |
| `true && expression` | Yes |
| `false \|\| expression` | Yes |
| `true \|\| expression` | No |

For boolean operands, `&` and `|` also calculate AND and OR, but evaluate both operands. Matching final truth values do not imply matching execution behavior.

```rust
fn probe(label: &str) -> bool {
    println!("called: {label}");
    true
}

fn main() {
    println!("lazy={}", false && probe("lazy"));
    println!("eager={}", false & probe("eager"));
}
```

Expected output:

```text
lazy=false
called: eager
eager=false
```

The first call to `probe` never happens. The second happens before its enclosing `println!` can display the result.

**Put a guard before the operation it protects.**

```rust
fn safe_check(divisor: i32) -> bool {
    divisor != 0 && 10 / divisor > 1
}
```

`safe_check(0)` returns `false` without dividing; `safe_check(2)` returns `true`. Replacing `&&` with `&` forces the division to run for zero and causes a panic. Reversing the two operands also attempts division before checking the divisor.

When reading a condition, track both its final value and its evaluation path. Output, updates, or errors inside a skipped operand do not occur. This is why a boolean expression can act as a guard, rather than merely calculate a truth value.

Related: [[07 Numeric Operations]]
