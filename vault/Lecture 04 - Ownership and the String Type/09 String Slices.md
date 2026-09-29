# 09 String Slices

[[Lecture 04 Overview|Lecture 4 overview]] · [[Lecture 04 - Ownership and the String Type/90 Code Map|Code map]]

## Borrowing a region of text

A string slice, `&str`, borrows text without copying bytes. It conceptually contains a pointer and byte length, connecting its validity to the borrowed data.

For `&text[start..end]`, the start is inclusive and the end exclusive. Both offsets count **bytes**. The range must satisfy `start <= end <= text.len()`, and both endpoints must lie on UTF-8 boundaries.

```rust
fn main() {
    let mut text = String::from("red blue");
    let word = &text[..3];
    // text.clear(); // Error: word is still needed below.
    println!("{word}"); // red
    text.clear(); // Allowed after word's last use.
    println!("{}", text.len()); // 0
}
```

`clear()` needs mutable access, which conflicts with an immutable slice that will still be used. Moving the mutation after the slice's final use resolves the conflict. A slice is not a saved copy that survives independently when its source changes or is dropped.

## Range forms

| Expression | Borrowed region |
| --- | --- |
| `&text[..end]` | Beginning to end, excluding end |
| `&text[start..]` | Start through the remaining text |
| `&text[..]` | Entire text |
| `&text[i..i]` | Empty region at a valid boundary |

A literal such as `"red"` is already `&str`. Invalid byte boundaries cause a runtime panic, covered in [[13 UTF-8 and String Indexing]].

## Code reference

Practice file: `src/lectures/lec4/09_string_slices.rs`

```bash
cargo run --bin lec4_string_slices
```

[Rust Book: String Slices](https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices)
