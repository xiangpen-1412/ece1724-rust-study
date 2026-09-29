# 02 Constants and Static

[[00 Lecture 03 Overview|Lecture 3 overview]] · [[90 Code Map|Code map]]

## Core idea

An immutable local binding, a constant, and an immutable static all prevent ordinary reassignment, but they describe different things. Choose the declaration by its purpose, rather than treating the three keywords as interchangeable.

| Declaration | Meaning |
| --- | --- |
| `let value = 10;` | A local binding, immutable by default |
| `const LIMIT: i32 = 10;` | A named value evaluated at compile time |
| `static LANGUAGE: &str = "Rust";` | A value in fixed storage that exists throughout the program |

A `const` does not designate a single shared storage location. A `static` does. This is a distinction about the language model; it is not a promise that each local variable occupies a particular stack address.

## Rules

Both `const` and `static` declarations require explicit types and constant initializers. An expression such as `60 * 60` is valid because it can be evaluated at compile time. An arbitrary runtime function call or keyboard input is not a constant initializer.

Constant and static names conventionally use `SCREAMING_SNAKE_CASE`. A `const` cannot be declared with `mut`. The static used here is also immutable, so ordinary assignment to it is rejected. Mutable global storage introduces additional restrictions and is unnecessary for these examples.

## Worked example

```rust
const THRESHOLD: i32 = 10;
const SECONDS_PER_HOUR: i32 = 60 * 60;
static LANGUAGE: &str = "Rust";

fn main() {
    println!("{THRESHOLD}, {SECONDS_PER_HOUR}");
    let mut local_language = LANGUAGE;
    println!("Before: {local_language}");
    local_language = "Go";
    println!("Local: {local_language}; static: {LANGUAGE}");
}
```

Expected output:

```text
10, 3600
Before: Rust
Local: Go; static: Rust
```

The assignment changes `local_language`, not `LANGUAGE`. Declaring the local binding mutable does not make the static mutable.

## Pitfalls

```rust
// const LIMIT = 100; // Error: an explicit type is required.
// const mut LIMIT: i32 = 100; // Error: const cannot use mut.
// THRESHOLD = 5; // Error: a constant cannot be reassigned.
// LANGUAGE = "Go"; // Error: this static is immutable.
```

“Immutable” does not mean “computed at compile time.” A normal immutable `let` can hold a result calculated during execution. Conversely, constants are not limited to numeric literals: valid constant expressions may compute their values.

## Reference code

Source: `src/lectures/lec3/02_constants_and_static.rs`

```bash
cargo run --bin lec3_constants_static
```
