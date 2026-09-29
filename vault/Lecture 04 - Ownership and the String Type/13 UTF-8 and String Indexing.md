# 13 UTF-8 and String Indexing

[[Lecture 04 Overview|Lecture 4 overview]] · [[Lecture 04 - Ownership and the String Type/90 Code Map|Code map]]

## Bytes are not characters

String stores valid UTF-8: **1–4 bytes per Unicode scalar value**. A Rust `char` represents one scalar and occupies **4 bytes** as a value; its UTF-8 encoding can be shorter. A displayed character, or grapheme cluster, may contain several scalars.

`len()` counts bytes. `chars()` yields scalars; `bytes()` yields encoded `u8` values. Counting scalars is not necessarily counting visible characters.

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

## Distinguish indexing failures

Integer indexing such as `text[0]` fails to compile even for ASCII text. Rust does not interpret it as either byte or character indexing. `as_bytes()[0]` explicitly obtains a byte; `chars().nth(0)` returns an optional scalar. Finding the nth scalar requires iteration, not constant-time indexing.

Range slicing uses byte offsets and requires `start <= end <= text.len()`, with both endpoints on UTF-8 boundaries. Here the first scalar occupies bytes 0–2. `[..3]` works; `[..1]` compiles but panics when executed. `get(range)` instead returns `None` for invalid bounds or boundaries.

A compile error prevents execution. A panic occurs when execution reaches the offending operation; earlier statements may already have run.

## Quick check

Compare `"\u{E9}"` and `"e\u{301}"`. What do `len()` and `chars().count()` return?

**Answer:** Respectively `(2, 1)` and `(3, 2)`. Both represent one grapheme cluster, but the second uses a base letter plus a combining accent. Similar appearance does not imply identical bytes or scalar counts.

## Code reference

Practice file: `src/lectures/lec4/13_utf8_and_indexing.rs`

Run: `cargo run --bin lec4_utf8_and_indexing`

[Rust Book: String Indexing](https://doc.rust-lang.org/book/ch08-02-strings.html#indexing-into-strings)
