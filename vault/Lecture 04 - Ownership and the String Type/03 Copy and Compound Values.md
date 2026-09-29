# 03 Copy and Compound Values

[[Lecture 04 Overview|Lecture 4 overview]] · [[Lecture 04 - Ownership and the String Type/90 Code Map|Code map]]

`Copy` allows implicit duplication of a value, leaving the source usable. The criterion is the type's implementation, not where it is stored. Even a struct containing only integers is not automatically Copy. A type implementing `Drop` cannot also implement `Copy`.

| Type | Assignment behavior |
| --- | --- |
| `i32`, `f64`, `bool`, `char` | Copy |
| `String` | Move |
| `&str`, `&String` | Copy the shared reference, not the text |
| `(i32, bool)`, `[i32; 3]` | Copy: all components are Copy |
| `(String, i32)`, `[String; 2]` | Move: a component is not Copy |
| `&mut i32` | Not Copy, even though i32 is Copy |

```rust
let mut original = [1, 2, 3];
let snapshot = original;
original[0] = 9;
println!("{original:?} | {snapshot:?}"); // [9, 2, 3] | [1, 2, 3]

let record = (String::from("Mina"), 8);
let name = record.0;
println!("{name}: {}", record.1); // Mina: 8
// println!("{record:?}"); // Error: partially moved value.
```

The array is duplicated because its elements are Copy. Fixed array length alone is insufficient: an array of Strings would move.

The tuple demonstrates a **partial move**: moving its String field leaves the integer field usable, but the complete tuple is no longer available as a whole. If the tuple binding is mutable, restoring the missing field can make the whole tuple usable again.

Copying a shared reference creates another view of existing data; it does not provide a separate text buffer as String cloning does. See [[05 Shared References]].

## Code reference

Source: `src/lectures/lec4/03_copy_and_compound_values.rs`

Run: `cargo run --bin lec4_copy_and_compound_values`

[Official Copy documentation](https://doc.rust-lang.org/std/marker/trait.Copy.html)
