# 17 For Loops and Labels

[[Lecture 03 Overview|Lecture 3 overview]] · [[Lecture 03 - Basic Programming Concepts/90 Code Map|Code map]]

`for` obtains each item from an iterator and runs its body once per item. It manages traversal, so a separate counter update is usually unnecessary.

```rust
let values = [10, 20, 30];
let mut sum = 0;
for value in values {
    sum += value;
}
println!("{sum}"); // 60
```

This `i32` array is `Copy`, so `values` remains usable. Changing a local binding in `for mut value in values` does not change the original array. Do not assume the same copying behavior for arrays containing non-`Copy` elements such as `String`.

## Ranges and indices

| Expression | Items |
| --- | --- |
| `1..4` | `1, 2, 3` |
| `1..=4` | `1, 2, 3, 4` |
| `(1..4).rev()` | `3, 2, 1` |
| `4..1` | None; it is not a countdown |

Valid array indices come from `0..values.len()`. Including the upper endpoint introduces an out-of-bounds index. The exclusive range is naturally empty for an empty array, avoiding a potentially underflowing `len() - 1` calculation.

`values.iter().enumerate()` provides an index and a reference to each element. In the following integer example, the pattern `&value` copies the referenced integer into `value`:

```rust
let arr = [12, 7, 19, 15];
let mut maximum = arr[0];
let mut best_index = 0;
for (index, &value) in arr.iter().enumerate() {
    if value > maximum {
        maximum = value;
        best_index = index;
    }
}
println!("{best_index}, {maximum}"); // 2, 19
```

This reference example assumes a nonempty array. A subtle mistake is initializing the maximum from another array, such as `values[0]` while searching `arr`. That can appear correct for some inputs and fail for others. Initialize from the array actually being searched, as above. With `>`, equal later maxima do not replace the first one.

## Break, continue, and return

```rust
let mut visits = 0;
'rows: for row in 0..3 {
    for column in 0..3 {
        if row == 1 && column == 1 {
            break 'rows;
        }
        visits += 1;
    }
}
println!("{visits}"); // 4
```

Unlabeled `break;` exits only the innermost loop. `break 'rows;` exits the named outer loop. Four positions were counted before `(1, 1)` triggered the exit.

`continue;` skips the current iteration's remaining statements and requests the next item. `return` exits the entire function: the repository's `first_even` returns `2`, without checking later values.

Related: [[15 Loop Expressions]] · [[16 While Loops]]

## Code reference

Repository path: `src/lectures/lec3/17_for_loops.rs`

```bash
cargo run --bin lec3_for_loops
```

[Rust Book: Looping through a collection](https://doc.rust-lang.org/book/ch03-05-control-flow.html#looping-through-a-collection-with-for)
