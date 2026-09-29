# 13 Functions and Expressions

[[Lecture 03 Overview|Lecture 3 overview]] · [[Lecture 03 - Basic Programming Concepts/90 Code Map|Code map]]

A function's signature defines the types accepted from its caller and the value returned on normal completion. Parameters require explicit types. Omitting `-> Type` means a return type of `()`, not an inferred return type. Function names conventionally use `snake_case`; a visible function may be defined after its call.

```rust
fn add_numbers(left: i32, right: i32) -> i32 {
    left + right
}

fn main() {
    let total = add_numbers(7, 5);
    println!("{total}"); // 12
}
```

The final expression without a semicolon supplies the function body's value. `return left + right;` also works: `return` exits the whole function immediately. Its semicolon does not cancel that control-flow change.

## Expressions, semicolons, and unit

| Body or block | Result |
| --- | --- |
| `{ 12 }` | Integer `12` |
| `{ 12; }` | Unit `()` |
| `{ let base = 3; base * 4 }` | Integer `12` |
| `{ println!("12"); }` | Prints text; produces `()` |

Therefore, `fn f() -> i32 { 12; }` fails: normal completion produces unit instead of the required integer. Printing `12` does not return `12` to the caller.

A missing semicolon does not guarantee a numeric result. Assignment itself produces unit:

```rust
let mut destination = 0;
println!("{destination}"); // 0
let result = { destination = 9 };
println!("{destination}, {result:?}"); // 9, ()
```

## Parameter types and local changes

If a function expects `u8`, the literal in `show_byte(10)` can be inferred as `u8`. An existing `i32` variable is already typed and will not automatically convert, even when its value fits.

```rust
fn increment_copy(mut value: i32) -> i32 {
    value += 1;
    value
}

let original = 41;
let result = increment_copy(original);
println!("{original}, {result}"); // 41, 42
```

Here `i32` is copied. `mut` permits changes to the local parameter binding; it does not modify the caller's variable. Do not generalize this copying behavior to every Rust type.

When diagnosing an error, check the signature, then the type produced by each normal completion path. See [[14 If Expressions]] for paths that return early.

## Code reference

Repository path: `src/lectures/lec3/13_functions.rs`

```bash
cargo run --bin lec3_functions
```

[Rust Book: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
