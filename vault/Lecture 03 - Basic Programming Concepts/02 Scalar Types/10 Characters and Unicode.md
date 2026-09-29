# 10 Characters and Unicode

[[00 Lecture 03 Overview|Lecture 3 overview]] · [[90 Code Map|Code map]]

Source: `src/lectures/lec3/10_characters.rs` ([GitHub](https://github.com/xiangpen-1412/ece1724-rust-study/blob/main/src/lectures/lec3/10_characters.rs)).

Run: `cargo run --bin lec3_characters`

**Literal spelling determines the type.**

| Literal | Type | Meaning |
| --- | --- | --- |
| `'R'` | `char` | One Unicode scalar value |
| `"R"` | `&str` | String text |
| `b'R'` | `u8` | One byte, with numeric value 82 |

A `char` is not restricted to ASCII. `\u{4E2D}` identifies a valid non-ASCII scalar; `\u{1F600}` identifies an emoji scalar. A literal such as `'ab'` is invalid because it contains more than one scalar. A string literal cannot be assigned directly to a `char` variable.

**Keep three measurements separate: bytes, scalar values, and visible characters.**

```rust
fn main() {
    let text = "\u{4E2D}";
    let symbol = '\u{4E2D}';
    println!("{} {}", text.len(), text.chars().count());
    println!("{} {}", std::mem::size_of::<char>(), symbol.len_utf8());
}
```

Expected output:

```text
3 1
4 3
```

`str::len()` counts UTF-8 bytes; `chars().count()` counts Unicode scalar values. A Rust `char` occupies four bytes in memory, while that scalar's UTF-8 encoding uses between one and four bytes. Its stored size and its encoded length are different facts.

For `"A"`, the byte/scalar counts are 1/1. For `"\u{1F600}"`, they are 4/1. The UTF-8 bytes for the first example are `[228, 184, 173]`.

**Similar appearance does not guarantee identical strings.**

```rust
let precomposed = "\u{E9}";
let combining = "e\u{301}";
println!("{} {}", precomposed.len(), precomposed.chars().count());
println!("{} {}", combining.len(), combining.chars().count());
println!("{}", precomposed == combining);
```

Inside `main`, this prints `2 1`, then `3 2`, then `false`. The second string combines a letter and an accent. String equality does not automatically normalize these different encodings. A displayed character, or grapheme cluster, may contain multiple scalar values; `chars().count()` is therefore not a general visible-character count.

An integer index such as `text[0]` is invalid for strings. `text.chars().next()` returns `Some(char)` for nonempty text and `None` for empty text. Finally, `count()` consumes its iterator: after `let chars = text.chars();`, calling `chars.count()` twice is invalid. Obtain a fresh iterator with another `text.chars()` call.

Related: [[06 Integer Types and Overflow]]
