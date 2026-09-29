# 12 Arrays

[[00 Lecture 03 Overview|Lecture 3 overview]] · [[90 Code Map|Code map]]

## Core idea

An array has a fixed number of elements of one type. `[i32; 4]` and `[i32; 3]` are different types because the length is part of the type. Elements are stored contiguously; the array type does not require every array value to live on the stack.

```rust
let values: [i32; 4] = [10, 20, 30, 40];
println!("{} {}", values.len(), values[2]);
```

Expected output: `4 30`. Indices start at zero, so the valid indices here are `0`, `1`, `2`, and `3`.

## Initialization and modification

| Expression | Result |
| --- | --- |
| `[3, 5]` | Two elements: 3 and 5 |
| `[3; 5]` | Five copies of integer 3 |
| `let empty: [i32; 0] = [];` | An empty integer array |

```rust
let mut scores = [60, 70, 80];
scores[1] = 95;
println!("{scores:?}");
```

The result is `[60, 95, 80]`. Mutability allows element updates; it does not allow resizing, adding an element with `push`, or assigning a different-length array to this binding.

## Bounds and index types

An ordinary integer array index has type `usize`. An explicitly typed `i32` index is not accepted automatically.

```rust
let values = [10, 20, 30, 40];
println!("{:?}", values.get(3));
println!("{:?}", values.get(4));
```

The results are `Some(40)` and `None`. `get` returns an optional reference rather than panicking for an invalid index. Full `Option` handling is a later topic.

Direct indexing with an invalid runtime index panics, including in release builds. A statically obvious out-of-bounds expression may instead be rejected during compilation. Do not treat compilation success as proof that every runtime index will be valid.

For iteration over indices, use `0..values.len()`. The inclusive range `0..=values.len()` includes one invalid index. For an empty array, avoid computing `len() - 1`, which can underflow.

## Two-dimensional arrays

```rust
let mut grid: [[i32; 3]; 2] = [[0; 3]; 2];
grid[1][2] = 9;
println!("{grid:?}");
```

Expected output: `[[0, 0, 0], [0, 0, 9]]`. There are two rows, each containing three columns. Check row and column bounds before a runtime access; check the row first before using it to access a row.

## Code

Repository path: `src/lectures/lec3/12_arrays.rs`

[Open the source on GitHub](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/12_arrays.rs)

```bash
cargo run --bin lec3_arrays
cargo run --bin lec3_arrays -- 2
cargo run --bin lec3_arrays -- 4
```

The last two commands demonstrate checked access with a runtime index. The existing source also provides an optional `--panic` experiment; it is not needed for normal review.

Related: [[06 Integer Types and Overflow]] · [[11 Tuples]] · [[17 For Loops and Labels]]
