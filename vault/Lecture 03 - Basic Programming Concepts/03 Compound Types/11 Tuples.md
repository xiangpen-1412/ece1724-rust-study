# 11 Tuples

[[00 Lecture 03 Overview|Lecture 3 overview]] · [[90 Code Map|Code map]]

## Core idea

A tuple groups a fixed number of values in a fixed order. Each field has its own type, so different positions can hold different types. Both field types and their order are part of the tuple's type.

```rust
let record: (i32, f64, u8) = (120, 2.5, 3);
let (count, price, level) = record;
println!("{count}, {price}, {level}");
println!("{}", record.1);
```

Expected output: `120, 2.5, 3`, followed by `2.5`.

Use `.0`, `.1`, and so on for fixed-position field access. A tuple is not indexed with `record[index]`. Destructuring must match the tuple's shape; `_` ignores a position, as in `let (_, price, _) = record;`.

## Mutability and destructuring

```rust
let mut position = (2, 4);
position.0 = 5;
let (mut row, column) = position;
row += 4;
println!("{row}, {column}; {position:?}");
```

The result is `9, 4; (5, 4)`. The new `row` binding is separate from the original tuple field. `mut` on `position` does not automatically make bindings created by destructuring mutable.

The integer fields are `Copy`, which is why both the destructured values and original tuple can still be used. Do not apply that conclusion to a tuple containing `String`: extracting or assigning such values can move them. The ownership rules belong to the next lecture.

`mut` allows changes that fit the existing type; it cannot add a field or replace an integer field with a float.

## One field, no fields, and block values

| Expression | Meaning |
| --- | --- |
| `(42)` | Parenthesized integer |
| `(42,)` | One-element tuple |
| `()` | Unit: a type with exactly one value |
| `{ 7 }` | Block whose value is integer 7 |
| `{ 7; }` | Block whose value is unit |

Unit is a real value, not the absence of all possible values. A function without an explicit return type returns `()`. Use `{:?}` to print unit or an ordinary tuple in these examples.

## Return several results

```rust
fn rectangle_metrics(width: i32, height: i32) -> (i32, i32) {
    (width * height, 2 * (width + height))
}
```

Calling `rectangle_metrics(3, 4)` returns `(12, 14)`. The caller can destructure the result into `area` and `perimeter`.

## Code

Repository path: `src/lectures/lec3/11_tuples.rs`

[Open the source on GitHub](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/11_tuples.rs)

```bash
cargo run --bin lec3_tuples
```

Related: [[03 Scope and Shadowing]] · [[12 Arrays]] · [[13 Functions and Expressions]]
