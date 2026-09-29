# 01 Variables and Mutability

[[00 Lecture 03 Overview|Lecture 3 overview]] · [[90 Code Map|Code map]]

## Core idea

`let` introduces a binding: a name associated with a value. A binding is immutable unless its declaration includes `mut`. This makes changes visible in the source: when a value must change during execution, the declaration explicitly permits it.

Mutability answers whether an existing binding can be assigned another value. It does not change the binding's type, and it is separate from creating another binding with the same name.

## Rules

- `let score = 10;` creates an immutable binding.
- `let mut score = 10;` permits later assignments such as `score = 15`.
- `score += 5` also changes the existing value, so it requires a mutable binding.
- The new value must have the binding's existing type. `mut` does not allow an integer binding to become a floating-point binding.
- A new `let` with an existing name is shadowing. See [[03 Scope and Shadowing]].

## Worked example

```rust
fn main() {
    let limit = 20;
    let mut score = 10;

    score = score + 5;
    println!("Score: {score}; limit: {limit}");

    score += 2;
    println!("Updated score: {score}");
}
```

Expected output:

```text
Score: 15; limit: 20
Updated score: 17
```

The right side of `score = score + 5` reads the current value, computes 15, and assigns that result to the existing binding. The statement does not create another variable.

## Pitfalls

Removing `mut` makes both assignments invalid; this is a compile-time error, rather than a failure encountered after printing something. Assigning the same value again still counts as assignment and requires mutability.

```rust
let mut value = 10;
// value = 10.5; // Error: the binding has an integer type.
```

Conversely, adding `mut` everywhere does not improve correctness. A binding that is never changed may produce an `unused_mut` warning. A warning is different from an error that prevents compilation.

An immutable local variable is also not automatically a `const` item. Both prevent ordinary reassignment, but constants have additional declaration and initialization rules; see [[02 Constants and Static]].

## Reference code

Source: `src/lectures/lec3/01_variables_and_mutability.rs`

```bash
cargo run --bin lec3_variables
```
