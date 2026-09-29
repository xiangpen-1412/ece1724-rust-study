# 10 str Parameters and Borrowed Results

[[Lecture 04 Overview|Lecture 4 overview]] · [[Lecture 04 - Ownership and the String Type/90 Code Map|Code map]]

## Accept text through a slice

Prefer `&str` when a function only needs to read text. It accepts string literals, slices, and borrowed Strings. Rust can convert `&String` to `&str` at the call; it does not copy the text or transfer ownership.

```rust
fn first_word(text: &str) -> &str {
    for (index, &byte) in text.as_bytes().iter().enumerate() {
        if byte == b' ' {
            return &text[..index];
        }
    }
    text
}

fn main() {
    let owned = String::from("red blue");
    let first = first_word(&owned);
    let second = first_word(&owned[4..]);
    let literal = first_word("green yellow");
    println!("{first} {second} {literal}"); // red blue green
}
```

The returned `&str` borrows the input, so the source must remain valid while the result is used. Returning from the function does not end this requirement. The function allocates no new String.

The ASCII-space search returns the entire input when no space exists. An empty input or leading space produces an empty slice. See [[08 First Word and Stale Indices]] for the loop.

## Borrowed versus independent results

A returned `usize` does not keep the source borrowed. A returned `&str` does: modifying or moving the original String conflicts with a later use of that slice.

For an independent result, `first_word(&owned).to_owned()` creates a String containing copied text. Calling `.clone()` on an `&str` instead copies its reference, not the borrowed bytes.

## Code reference

Practice file: `src/lectures/lec4/10_str_parameters.rs`

```bash
cargo run --bin lec4_str_parameters
```

[Rust Book: String Slices as Parameters](https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices-as-parameters)
