# 05 Type Inference

[[00 Lecture 03 Overview|Lecture 3 overview]] · [[90 Code Map|Code map]]

## Core idea

Rust is statically typed, but many local bindings do not need explicit annotations. The compiler determines their types from the initializer and the ways the values are used. Inference selects a type during compilation; it does not make a variable change type while the program runs.

Later uses can constrain an earlier declaration. Only when no stronger constraint determines the numeric type do integer literals default to `i32` and floating-point literals default to `f64`.

## Worked example

```rust
fn main() {
    let count = 10;
    let price = 3.5;
    let n = 300;
    let m: u16 = n;

    println!("count: {}", std::any::type_name_of_val(&count));
    println!("price: {}", std::any::type_name_of_val(&price));
    println!("n: {}; m: {m}", std::any::type_name_of_val(&n));
}
```

Expected output:

```text
count: i32
price: f64
n: u16; m: 300
```

The annotation on `m` constrains `n` to be `u16` too. Rust does not first create an `i32` and then silently convert it. Changing `m` to `u8` fails because the literal 300 cannot fit in that inferred type.

## Inference is not conversion

```rust
let inferred = 10;
let destination: u8 = inferred;

let fixed: i32 = 10;
// let invalid: u8 = fixed; // Error: expected u8, found i32.
let converted = fixed as u8;
println!("{destination}, {converted}"); // 10, 10
```

The first declaration leaves room for inference to select `u8`. The explicit `i32` annotation in the second case does not. A value fitting into a destination's numeric range is insufficient for implicit assignment between different integer types.

The cast shown preserves 10, but a narrowing `as` cast generally does not check whether a value fits. Successful compilation does not guarantee preservation of the numeric value.

## Pitfalls

A literal suffix also fixes a type: `25_u64` is explicitly `u64`. An annotation and a suffix must agree with the uses of that value.

`mut` allows reassignment without allowing type changes. Using another `let` can introduce a new binding with another type; that is shadowing, not dynamic typing. See [[01 Variables and Mutability]] and [[03 Scope and Shadowing]].

Inference also needs enough information. For example, `"42".parse()` alone does not identify the desired numeric type; specify a destination type or use `parse::<u32>()`.

## Reference code

Source: `src/lectures/lec3/05_type_inference.rs`

```bash
cargo run --bin lec3_type_inference
```
