# 11 Array Slices

[[Lecture 04 Overview|Lecture 4 overview]] · [[Lecture 04 - Ownership and the String Type/90 Code Map|Code map]]

## Borrowing several elements

Array slices borrow elements instead of text. `&[i32]` is a shared integer slice; `&mut [i32]` permits changes to the borrowed elements. Neither owns a copied array.

Unlike `[i32; 4]`, a slice type does not include its length. Its length is carried with the reference, allowing a function taking `&[i32]` to accept different-sized regions.

```rust
fn main() {
    let mut values = [10, 20, 30, 40];
    let middle: &[i32] = &values[1..3];
    println!("{middle:?} {}", middle.len()); // [20, 30] 2

    let edit: &mut [i32] = &mut values[1..3];
    edit[0] += 1;
    println!("{values:?}"); // [10, 21, 30, 40]
}
```

The shared borrow ends after `middle`'s final use, allowing the mutable borrow. The final print similarly follows the last use of `edit`. Adding a later use of `middle` after creating and using `edit` would introduce a conflict.

## Indices belong to the slice

Slice indices start at zero. Here `edit[0]` refers to `values[1]`. Array-slice lengths and offsets count **elements**, unlike string-slice offsets, which count UTF-8 bytes.

`&values[..]` borrows the whole array. A range excludes its upper endpoint; `&values[1..1]` is empty. An invalid runtime index or range panics. Mutable access permits element replacement, but does not resize the array or add elements.

Copying a shared slice reference still accesses the original elements; it creates no independent array.

## Note

1. Use of owner before the end of reference

```rust
fn test() {  
    let mut values = [10, 20, 30, 40];  
    let snapshot = values;  
  
    let part = &mut values[1..3];  
  
    part[0] += 5; 
    // error: last use of borrowed reference still exists 
    // println!("{values:?}");  
    part[1] *= 2;  
  
    println!("{part:?}");  
    println!("{snapshot:?}");  
    println!("{values:?}");  
}
```
## Code reference

Practice file: `src/lectures/lec4/11_array_slices.rs`

```bash
cargo run --bin lec4_array_slices
```

[Rust Book: Other Slices](https://doc.rust-lang.org/book/ch04-03-slices.html#other-slices)
