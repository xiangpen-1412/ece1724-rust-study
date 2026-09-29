# 13 UTF-8 and String Indexing

[[Lecture 04 Overview|Lecture 4 overview]] · [[Lecture 04 - Ownership and the String Type/90 Code Map|Code map]]

## Bytes are not characters

String stores valid UTF-8 bytes. One Unicode scalar value uses **1–4 bytes**, correcting the lecture's one-or-two-byte simplification. A Rust `char` represents one scalar; one visible character may contain several scalars.

`len()` counts bytes; `chars()` iterates over scalars; `bytes()` iterates over encoded `u8` values.

```rust
fn main() {
    let text = String::from("\u{4E2D}A");
    println!("{} {}", text.len(), text.chars().count()); // 4 2
    println!("{}", &text[..3]); // U+4E2D alone
    println!("{:?}", text.get(..1)); // None

    for byte in text.bytes() {
        print!("{byte} ");
    }
    println!(); // 228 184 173 65

    // let first = text[0]; // Compile error.
    // let broken = &text[..1]; // Runtime panic.
}
```

## Distinguish two failures

Integer indexing such as `text[0]` fails to compile, even for ASCII text. Rust does not choose between byte, scalar, and visible-character indexing.

Range slicing is supported, but offsets count bytes. Both endpoints must be valid UTF-8 boundaries, with `start <= end <= text.len()`. Ending inside an encoded scalar causes a runtime panic. In the example, the first scalar occupies bytes 0–2, so `[..3]` works while `[..1]` does not.

`get(range)` returns `None` instead of panicking for invalid bounds or boundaries. `as_bytes()[0]` accesses one byte; it does not necessarily obtain a complete character.

A compilation error prevents the program from running at all. A panic occurs when execution reaches the failing operation.

## Code reference

No dedicated UTF-8 practice file exists yet. Revisit boundaries in `src/lectures/lec4/09_string_slices.rs`.

[Rust Book: String Indexing](https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings)
