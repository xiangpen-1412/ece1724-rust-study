# 08 First Word and Stale Indices

[[Lecture 04 Overview|Lecture 4 overview]] · [[Lecture 04 - Ownership and the String Type/90 Code Map|Code map]]

## The problem with an independent index

An index describes a position without keeping a connection to its String. Returning `usize` gives the caller an independent integer. Changing the text afterward can make that position meaningless, even though the integer remains valid.

```rust
fn first_word_end(text: &String) -> usize {
    for (index, &byte) in text.as_bytes().iter().enumerate() {
        if byte == b' ' {
            return index;
        }
    }
    text.len()
}

fn main() {
    let mut text = String::from("red blue");
    let boundary = first_word_end(&text);
    text.clear();
    println!("{boundary} {}", text.len()); // 3 0
}
```

The borrow used by the call does not remain active through `boundary`: an integer does not borrow text. Consequently, `clear()` compiles. The saved boundary no longer identifies a word in the empty String.

## Reading the loop

`as_bytes()` returns a borrowed `&[u8]`, not a newly allocated array. `iter()` yields byte references; `enumerate()` adds each byte's position. The pattern `&byte` copies the referenced `u8`. `b' '` denotes one ASCII-space byte.

Without a space, the function returns the byte length. Empty input and a leading space both produce zero. Tabs are not separators here.

A stale index need not be out of bounds: replacing the text can leave it in bounds but pointing to the wrong word. [[09 String Slices|Returning a slice]] lets borrowing rules protect the relationship instead.

## Code reference

Practice file: `src/lectures/lec4/08_first_word_index.rs`

```bash
cargo run --bin lec4_first_word_index
```

[Rust Book: The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html)
