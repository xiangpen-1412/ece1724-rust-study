# 14 If Expressions

[[Lecture 03 Overview|Lecture 3 overview]] · [[Lecture 03 - Basic Programming Concepts/90 Code Map|Code map]]

An ordinary `if` condition must produce `bool`: write `number > 0`, not just `number`. Assignment such as `number = 0` produces unit, so it is not a valid condition either.

An `if / else if / else` chain executes only its first matching branch. With `score = 95`, checking `score >= 60` before `score >= 90` selects the broad condition first. Put the more specific grade boundary first when that is the intended classification.

## Selecting a value

```rust
let enabled = true;
let amount = if enabled { 10 } else { 20 };
println!("{amount}"); // 10
```

Each branch's tail expression supplies its value. The final semicolon terminates the outer `let` statement; it does not turn `amount` into unit.

Branches that complete normally must have compatible result types. These combinations fail:

| Branch results | Problem |
| --- | --- |
| `10` and `"twenty"` | Integer versus string reference |
| `10` and `20;` | Integer versus unit |
| `10` and `20.0` | Integer versus floating-point value |

Both branches are type-checked even in `if true { ... } else { ... }`. A value-producing integer example also needs an `else`; a no-`else` `if` does not supply an integer when its condition is false. Two branches that merely print can agree on unit.

## Boundaries and early return

```rust
fn scale_small(number: i32) -> i32 {
    if number > -10 && number < 10 {
        number * 10
    } else {
        number / 2
    }
}
```

| Input | Result |
| --- | --- |
| `9` | `90` |
| `10` | `5` |
| `-9` | `-90` |
| `-10` | `-5` |
| `-11` | `-5` |

The endpoints are excluded. Integer division truncates toward zero. Write the two comparisons with `&&`; Rust does not accept the mathematical chain `-10 < number < 10`.

```rust
fn is_divisible_by(dividend: u32, divisor: u32) -> bool {
    let remainder = if divisor == 0 {
        return false;
    } else {
        dividend % divisor
    };
    remainder == 0
}
```

This example defines zero-divisor input as `false`. That branch exits the entire function before `% 0`. It does not supply a Boolean value to `remainder`: only the continuing branch supplies that `u32`. This differs from a branch that completes normally with unit.

Related: [[13 Functions and Expressions]] · [[15 Loop Expressions]]

## Code reference

Repository path: `src/lectures/lec3/14_if_expressions.rs`

```bash
cargo run --bin lec3_if_expressions
```

[Rust Reference: If expressions](https://doc.rust-lang.org/reference/expressions/if-expr.html)
